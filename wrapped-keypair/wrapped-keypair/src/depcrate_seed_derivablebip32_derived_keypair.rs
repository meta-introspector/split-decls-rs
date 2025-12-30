// Generated macro for bip32_derived_keypair (function)
macro_rules! Depcrate_seed_derivablebip32_derived_keypair {
() => {
// Module: crate::seed_derivable
// Provides: {"bip32_derived_keypair"}
// Dependencies: {}
# [doc = " Generates a Keypair using Bip32 Hierarchical Derivation"] fn bip32_derived_keypair (seed : & [u8] , derivation_path : DerivationPath ,) -> Result < Keypair , Bip32Error > { let extended = ed25519_dalek_bip32 :: ExtendedSigningKey :: from_seed (seed) . and_then (| extended | extended . derive (& derivation_path)) ? ; Ok (Keypair (extended . signing_key)) }
};
}
