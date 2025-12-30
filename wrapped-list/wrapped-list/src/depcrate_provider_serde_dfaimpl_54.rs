// Generated macro for impl_54 (impl)
macro_rules! Depcrate_provider_serde_dfaimpl_54 {
() => {
// Module: crate::provider::serde_dfa
// Provides: {"impl_54"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl databake :: Bake for SerdeDFA < '_ > { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { env . insert ("icu_list") ; let le_bytes = databake :: Bake :: bake (& self . deref () . to_bytes_little_endian () . as_slice () , env) ; let be_bytes = databake :: Bake :: bake (& self . deref () . to_bytes_big_endian () . as_slice () , env) ; databake :: quote ! { unsafe { icu_list :: provider :: SerdeDFA :: from_dfa_bytes_unchecked (if cfg ! (target_endian = "little") { # le_bytes } else { # be_bytes }) } } } }
};
}
