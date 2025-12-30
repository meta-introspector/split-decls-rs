// Generated macro for impl_57 (impl)
macro_rules! Depcrate_provider_serde_dfaimpl_57 {
() => {
// Module: crate::provider::serde_dfa
// Provides: {"impl_57"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'data > SerdeDFA < 'data > { # [doc = " Deserializes to `Option<Self>`. Will return `None` for non-human-readable serialization"] # [doc = " formats on big-endian systems, as `regex_automata` serialization is endian-sensitive."] pub fn maybe_deserialize < 'de : 'data , D > (deserializer : D) -> Result < Option < Self > , D :: Error > where D : serde :: de :: Deserializer < 'de > , { use serde :: Deserialize ; # [cfg (feature = "serde_human")] if deserializer . is_human_readable () { use alloc :: string :: ToString ; use serde :: de :: Error ; return SerdeDFA :: new (alloc :: borrow :: Cow :: < str > :: deserialize (deserializer) ?) . map (Some) . map_err (| e | D :: Error :: custom (e . to_string ())) ; } let dfa_bytes = VarZeroCow :: < [u8] > :: deserialize (deserializer) ? ; if cfg ! (target_endian = "big") { return Ok (None) ; } DFA :: from_bytes (& dfa_bytes) . map_err (| _e | { use serde :: de :: Error ; D :: Error :: custom ("Invalid DFA bytes") }) ? ; Ok (Some (SerdeDFA { dfa_bytes , # [cfg (feature = "serde_human")] pattern : None , })) } }
};
}
