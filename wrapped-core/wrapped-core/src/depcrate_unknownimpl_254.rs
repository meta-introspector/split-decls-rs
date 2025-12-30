// Generated macro for impl_254 (impl)
macro_rules! Depcrate_unknownimpl_254 {
() => {
// Module: crate::unknown
// Provides: {"impl_254"}
// Dependencies: {}
impl Drop for IUnknown { fn drop (& mut self) { unsafe { (self . vtable () . Release) (core :: mem :: transmute_copy (self)) ; } } }
};
}
