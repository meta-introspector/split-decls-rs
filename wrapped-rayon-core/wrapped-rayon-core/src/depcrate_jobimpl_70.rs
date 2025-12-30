// Generated macro for impl_70 (impl)
macro_rules! Depcrate_jobimpl_70 {
() => {
// Module: crate::job
// Provides: {"impl_70"}
// Dependencies: {}
impl < BODY > Job for HeapJob < BODY > where BODY : FnOnce () + Send , { unsafe fn execute (this : * const ()) { unsafe { let this = Box :: from_raw (this as * mut Self) ; (this . job) () ; } } }
};
}
