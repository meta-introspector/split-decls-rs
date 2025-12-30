// Generated macro for cert_is_self_issued (function)
macro_rules! Depcrate_certificatecert_is_self_issued {
() => {
// Module: crate::certificate
// Provides: {"cert_is_self_issued"}
// Dependencies: {}
pub (crate) fn cert_is_self_issued (cert : & Certificate < '_ >) -> bool { cert . issuer () == cert . subject () }
};
}
