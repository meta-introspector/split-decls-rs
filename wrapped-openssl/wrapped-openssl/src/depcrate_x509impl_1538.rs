// Generated macro for impl_1538 (impl)
macro_rules! Depcrate_x509impl_1538 {
() => {
// Module: crate::x509
// Provides: {"impl_1538"}
// Dependencies: {}
impl ToOwned for X509Ref { type Owned = X509 ; fn to_owned (& self) -> X509 { unsafe { X509_up_ref (self . as_ptr ()) ; X509 :: from_ptr (self . as_ptr ()) } } }
};
}
