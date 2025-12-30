// Generated macro for impl_1130 (impl)
macro_rules! Depcrate_signatureimpl_1130 {
() => {
// Module: crate::signature
// Provides: {"impl_1130"}
// Dependencies: {}
impl Clone for Signature < 'static > { fn clone (& self) -> Signature < 'static > { let mut raw = ptr :: null_mut () ; let rc = unsafe { raw :: git_signature_dup (& mut raw , & * self . raw) } ; assert_eq ! (rc , 0) ; unsafe { Binding :: from_raw (raw) } } }
};
}
