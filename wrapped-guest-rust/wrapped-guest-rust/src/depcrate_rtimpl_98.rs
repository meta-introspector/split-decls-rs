// Generated macro for impl_98 (impl)
macro_rules! Depcrate_rtimpl_98 {
() => {
// Module: crate::rt
// Provides: {"impl_98"}
// Dependencies: {}
impl Drop for Cleanup { fn drop (& mut self) { unsafe { for i in 0 .. self . layout . size () { * self . ptr . add (i) . as_ptr () = 0xff ; } alloc :: alloc :: dealloc (self . ptr . as_ptr () , self . layout) ; } } }
};
}
