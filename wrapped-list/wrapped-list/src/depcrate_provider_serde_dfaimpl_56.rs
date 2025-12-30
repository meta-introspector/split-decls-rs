// Generated macro for impl_56 (impl)
macro_rules! Depcrate_provider_serde_dfaimpl_56 {
() => {
// Module: crate::provider::serde_dfa
// Provides: {"impl_56"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl serde :: Serialize for SerdeDFA < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { # [cfg (feature = "serde_human")] if serializer . is_human_readable () { return self . pattern . as_ref () . map (| pattern | pattern . serialize (serializer)) . unwrap_or_else (| | { use serde :: ser :: Error ; Err (S :: Error :: custom ("cannot serialize a binary-deserialized SerdeDFA to JSON" ,)) }) ; } serializer . serialize_bytes (& self . deref () . to_bytes_little_endian ()) } }
};
}
