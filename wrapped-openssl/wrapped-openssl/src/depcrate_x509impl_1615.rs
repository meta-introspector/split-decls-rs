// Generated macro for impl_1615 (impl)
macro_rules! Depcrate_x509impl_1615 {
() => {
// Module: crate::x509
// Provides: {"impl_1615"}
// Dependencies: {}
impl X509AlgorithmRef { # [doc = " Returns the ASN.1 OID of this algorithm."] pub fn object (& self) -> & Asn1ObjectRef { unsafe { let mut oid = ptr :: null () ; X509_ALGOR_get0 (& mut oid , ptr :: null_mut () , ptr :: null_mut () , self . as_ptr ()) ; Asn1ObjectRef :: from_const_ptr_opt (oid) . expect ("algorithm oid must not be null") } } }
};
}
