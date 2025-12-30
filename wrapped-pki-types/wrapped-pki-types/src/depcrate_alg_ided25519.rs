// Generated macro for ED25519 (const)
macro_rules! Depcrate_alg_idED25519 {
() => {
// Module: crate::alg_id
// Provides: {"ED25519"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `ED25519`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # ed25519"] # [doc = " OBJECT_IDENTIFIER { 1.3.101.112 }"] # [doc = " ```"] pub const ED25519 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-ed25519.der")) ;
};
}
