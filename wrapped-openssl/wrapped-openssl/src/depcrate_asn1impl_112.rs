// Generated macro for impl_112 (impl)
macro_rules! Depcrate_asn1impl_112 {
() => {
// Module: crate::asn1
// Provides: {"impl_112"}
// Dependencies: {}
impl Ord for Asn1IntegerRef { fn cmp (& self , other : & Self) -> Ordering { let res = unsafe { ffi :: ASN1_INTEGER_cmp (self . as_ptr () , other . as_ptr ()) } ; res . cmp (& 0) } }
};
}
