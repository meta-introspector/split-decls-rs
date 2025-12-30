// Generated macro for impl_1607 (impl)
macro_rules! Depcrate_x509impl_1607 {
() => {
// Module: crate::x509
// Provides: {"impl_1607"}
// Dependencies: {}
impl DistPointRef { # [doc = " Returns the name of this distribution point if it exists"] pub fn distpoint (& self) -> Option < & DistPointNameRef > { unsafe { DistPointNameRef :: from_const_ptr_opt ((* self . as_ptr ()) . distpoint) } } }
};
}
