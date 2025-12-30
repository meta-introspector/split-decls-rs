// Generated macro for impl_253 (impl)
macro_rules! Depcrate_unknownimpl_253 {
() => {
// Module: crate::unknown
// Provides: {"impl_253"}
// Dependencies: {}
impl Clone for IUnknown { fn clone (& self) -> Self { unsafe { (self . vtable () . AddRef) (core :: mem :: transmute_copy (self)) ; } Self (self . 0) } }
};
}
