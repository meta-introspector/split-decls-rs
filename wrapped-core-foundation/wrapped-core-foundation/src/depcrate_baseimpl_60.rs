// Generated macro for impl_60 (impl)
macro_rules! Depcrate_baseimpl_60 {
() => {
// Module: crate::base
// Provides: {"impl_60"}
// Dependencies: {}
impl PartialEq for CFType { # [inline] fn eq (& self , other : & CFType) -> bool { unsafe { CFEqual (self . as_CFTypeRef () , other . as_CFTypeRef ()) != 0 } } }
};
}
