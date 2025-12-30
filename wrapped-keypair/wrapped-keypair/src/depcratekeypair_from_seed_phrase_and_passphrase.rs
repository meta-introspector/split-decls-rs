// Generated macro for keypair_from_seed_phrase_and_passphrase (function)
macro_rules! Depcratekeypair_from_seed_phrase_and_passphrase {
() => {
// Module: crate
// Provides: {"keypair_from_seed_phrase_and_passphrase"}
// Dependencies: {}
pub fn keypair_from_seed_phrase_and_passphrase (seed_phrase : & str , passphrase : & str ,) -> Result < Keypair , Box < dyn core :: error :: Error > > { keypair_from_seed (& generate_seed_from_seed_phrase_and_passphrase (seed_phrase , passphrase ,)) }
};
}
