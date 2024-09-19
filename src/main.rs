use tari_common::configuration::Network;
use tari_common_types::tari_address::{TariAddress, TariAddressFeatures};
use tari_crypto::{keys::PublicKey, ristretto::RistrettoPublicKey};
fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();
    let (_, public_key) = RistrettoPublicKey::random_keypair(&mut rng);
    let (_, spend_public_key) = RistrettoPublicKey::random_keypair(&mut rng);
    let addr = TariAddress::new_dual_address(
        public_key,
        spend_public_key,
        Network::get_current_or_user_setting_or_default(),
        TariAddressFeatures::create_one_sided_only(),
    );
    println!("Address: {}", addr.to_base58());
}
