// Generated macro for keypair_from_seed_and_derivation_path (function)
macro_rules! Depcrate_seed_derivablekeypair_from_seed_and_derivation_path {
() => {
// Module: crate::seed_derivable
// Provides: {"keypair_from_seed_and_derivation_path"}
// Dependencies: {}
# [doc = " Generates a Keypair using Bip32 Hierarchical Derivation if derivation-path is provided;"] # [doc = " otherwise generates the base Bip44 Solana keypair from the seed"] pub fn keypair_from_seed_and_derivation_path (seed : & [u8] , derivation_path : Option < DerivationPath > ,) -> Result < Keypair , Box < dyn error :: Error > > { let derivation_path = derivation_path . unwrap_or_default () ; bip32_derived_keypair (seed , derivation_path) . map_err (| err | err . to_string () . into ()) }
};
}
