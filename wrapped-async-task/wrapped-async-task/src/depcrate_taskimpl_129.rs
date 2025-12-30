// Generated macro for impl_129 (impl)
macro_rules! Depcrate_taskimpl_129 {
() => {
// Module: crate::task
// Provides: {"impl_129"}
// Dependencies: {}
impl < T , M > Drop for Task < T , M > { fn drop (& mut self) { let ptr = self . ptr . as_ptr () ; set_canceled (ptr) ; set_detached :: < T > (ptr) ; } }
};
}
