// Generated macro for impl_294 (impl)
macro_rules! Depcrate_reprimpl_294 {
() => {
// Module: crate::repr
// Provides: {"impl_294"}
// Dependencies: {}
impl Drop for Repr { # [inline] fn drop (& mut self) { if self . is_heap_allocated () { outlined_drop (self) } # [cold] fn outlined_drop (this : & mut Repr) { let heap_buffer = unsafe { this . as_mut_heap () } ; heap_buffer . dealloc () ; } } }
};
}
