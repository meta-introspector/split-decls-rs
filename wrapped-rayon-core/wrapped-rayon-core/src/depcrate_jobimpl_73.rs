// Generated macro for impl_73 (impl)
macro_rules! Depcrate_jobimpl_73 {
() => {
// Module: crate::job
// Provides: {"impl_73"}
// Dependencies: {}
impl < BODY > Job for ArcJob < BODY > where BODY : Fn () + Send + Sync , { unsafe fn execute (this : * const ()) { unsafe { let this = Arc :: from_raw (this as * mut Self) ; (this . job) () ; } } }
};
}
