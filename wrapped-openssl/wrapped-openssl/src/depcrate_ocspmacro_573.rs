// Generated macro for macro_573 (macro)
macro_rules! Depcrate_ocspmacro_573 {
() => {
// Module: crate::ocsp
// Provides: {"macro_573"}
// Dependencies: {}
bitflags ! { # [derive (Copy , Clone , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct OcspFlag : c_ulong { const NO_CERTS = ffi :: OCSP_NOCERTS as c_ulong ; const NO_INTERN = ffi :: OCSP_NOINTERN as c_ulong ; const NO_CHAIN = ffi :: OCSP_NOCHAIN as c_ulong ; const NO_VERIFY = ffi :: OCSP_NOVERIFY as c_ulong ; const NO_EXPLICIT = ffi :: OCSP_NOEXPLICIT as c_ulong ; const NO_CA_SIGN = ffi :: OCSP_NOCASIGN as c_ulong ; const NO_DELEGATED = ffi :: OCSP_NODELEGATED as c_ulong ; const NO_CHECKS = ffi :: OCSP_NOCHECKS as c_ulong ; const TRUST_OTHER = ffi :: OCSP_TRUSTOTHER as c_ulong ; const RESPID_KEY = ffi :: OCSP_RESPID_KEY as c_ulong ; const NO_TIME = ffi :: OCSP_NOTIME as c_ulong ; } }
};
}
