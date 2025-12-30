// Generated macro for ED448 (const)
macro_rules! Depcrate_alg_idED448 {
() => {
// Module: crate::alg_id
// Provides: {"ED448"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `ED448`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # ed448"] # [doc = " OBJECT_IDENTIFIER { 1.3.101.113 }"] # [doc = " ```"] pub const ED448 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-ed448.der")) ;
};
}
