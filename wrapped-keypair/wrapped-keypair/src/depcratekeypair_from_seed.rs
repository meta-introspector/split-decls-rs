// Generated macro for keypair_from_seed (function)
macro_rules! Depcratekeypair_from_seed {
() => {
// Module: crate
// Provides: {"keypair_from_seed"}
// Dependencies: {}
# [doc = " Constructs a `Keypair` from caller-provided seed entropy"] pub fn keypair_from_seed (seed : & [u8]) -> Result < Keypair , Box < dyn error :: Error > > { if seed . len () < ed25519_dalek :: SECRET_KEY_LENGTH { return Err ("Seed is too short" . into ()) ; } let secret_key = ed25519_dalek :: SecretKey :: try_from (& seed [.. ed25519_dalek :: SECRET_KEY_LENGTH]) ? ; Ok (Keypair (ed25519_dalek :: SigningKey :: from (secret_key))) }
};
}
