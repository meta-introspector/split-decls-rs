// Generated macro for TCFTypeRef (trait)
macro_rules! Depcrate_baseTCFTypeRef {
() => {
// Module: crate::base
// Provides: {"TCFTypeRef"}
// Dependencies: {}
# [doc = " Trait for all types which are Core Foundation reference types."] pub trait TCFTypeRef { fn as_void_ptr (& self) -> * const c_void ; unsafe fn from_void_ptr (ptr : * const c_void) -> Self ; }
};
}
