// Generated macro for impl_1609 (impl)
macro_rules! Depcrate_x509impl_1609 {
() => {
// Module: crate::x509
// Provides: {"impl_1609"}
// Dependencies: {}
impl DistPointNameRef { # [doc = " Returns the contents of this DistPointName if it is a fullname."] pub fn fullname (& self) -> Option < & StackRef < GeneralName > > { unsafe { if (* self . as_ptr ()) . type_ != 0 { return None ; } StackRef :: from_const_ptr_opt ((* self . as_ptr ()) . name . fullname) } } }
};
}
