// Generated macro for default_mgf1_sha1 (function)
macro_rules! Depcrate_paramsdefault_mgf1_sha1 {
() => {
// Module: crate::params
// Provides: {"default_mgf1_sha1"}
// Dependencies: {}
# [doc = " Default Mask Generation Function (MGF): SHA-1."] fn default_mgf1_sha1 < 'a > () -> AlgorithmIdentifier < AlgorithmIdentifierRef < 'a > > { AlgorithmIdentifier :: < AlgorithmIdentifierRef < 'a > > { oid : OID_MGF_1 , parameters : Some (SHA_1_AI) , } }
};
}
