// Generated macro for impl_5 (impl)
macro_rules! Depcrate_seed_derivableimpl_5 {
() => {
// Module: crate::seed_derivable
// Provides: {"impl_5"}
// Dependencies: {}
impl SeedDerivable for Keypair { fn from_seed (seed : & [u8]) -> Result < Self , Box < dyn error :: Error > > { keypair_from_seed (seed) } fn from_seed_and_derivation_path (seed : & [u8] , derivation_path : Option < DerivationPath > ,) -> Result < Self , Box < dyn error :: Error > > { keypair_from_seed_and_derivation_path (seed , derivation_path) } fn from_seed_phrase_and_passphrase (seed_phrase : & str , passphrase : & str ,) -> Result < Self , Box < dyn error :: Error > > { keypair_from_seed_phrase_and_passphrase (seed_phrase , passphrase) } }
};
}
