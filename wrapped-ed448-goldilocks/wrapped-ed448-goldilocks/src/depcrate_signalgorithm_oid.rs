// Generated macro for ALGORITHM_OID (const)
macro_rules! Depcrate_signALGORITHM_OID {
() => {
// Module: crate::sign
// Provides: {"ALGORITHM_OID"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] # [doc = " The OID for Ed448 as defined in [RFC8410 §2]"] pub const ALGORITHM_OID : pkcs8 :: ObjectIdentifier = pkcs8 :: ObjectIdentifier :: new_unwrap ("1.3.101.113") ;
};
}
