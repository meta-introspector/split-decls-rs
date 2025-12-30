// Generated macro for impl_1612 (impl)
macro_rules! Depcrate_x509impl_1612 {
() => {
// Module: crate::x509
// Provides: {"impl_1612"}
// Dependencies: {}
impl AccessDescriptionRef { # [doc = " Returns the access method OID."] pub fn method (& self) -> & Asn1ObjectRef { unsafe { Asn1ObjectRef :: from_ptr ((* self . as_ptr ()) . method) } } pub fn location (& self) -> & GeneralNameRef { unsafe { GeneralNameRef :: from_ptr ((* self . as_ptr ()) . location) } } }
};
}
