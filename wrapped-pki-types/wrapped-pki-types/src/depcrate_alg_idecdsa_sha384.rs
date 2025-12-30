// Generated macro for ECDSA_SHA384 (const)
macro_rules! Depcrate_alg_idECDSA_SHA384 {
() => {
// Module: crate::alg_id
// Provides: {"ECDSA_SHA384"}
// Dependencies: {}
# [doc = " AlgorithmIdentifier for `ecdsa-with-SHA384`."] # [doc = ""] # [doc = " This is:"] # [doc = ""] # [doc = " ```text"] # [doc = " # ecdsa-with-SHA384"] # [doc = " OBJECT_IDENTIFIER { 1.2.840.10045.4.3.3 }"] # [doc = " ```"] pub const ECDSA_SHA384 : AlgorithmIdentifier = AlgorithmIdentifier :: from_slice (include_bytes ! ("data/alg-ecdsa-sha384.der")) ;
};
}
