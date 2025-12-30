// Generated macro for ML_DSA_44 (const)
macro_rules! Depcrate_alg_idML_DSA_44 {
() => {
// Module: crate::alg_id
// Provides: {"ML_DSA_44"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `id-ml-dsa-44`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.3.17 }"] # [doc = " ```"] # [doc = ""] # [doc = " <https://www.ietf.org/archive/id/draft-ietf-lamps-dilithium-certificates-07.html#name-identifiers>"] pub const ML_DSA_44 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-ml-dsa-44.der")) ;
};
}
