// Generated macro for ML_DSA_65 (const)
macro_rules! Depcrate_alg_idML_DSA_65 {
() => {
// Module: crate::alg_id
// Provides: {"ML_DSA_65"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `id-ml-dsa-65`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.3.18 }"] # [doc = " ```"] # [doc = ""] # [doc = " <https://www.ietf.org/archive/id/draft-ietf-lamps-dilithium-certificates-07.html#name-identifiers>"] pub const ML_DSA_65 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-ml-dsa-65.der")) ;
};
}
