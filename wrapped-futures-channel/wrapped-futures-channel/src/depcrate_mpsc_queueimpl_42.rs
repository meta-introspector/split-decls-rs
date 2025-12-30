// Generated macro for impl_42 (impl)
macro_rules! Depcrate_mpsc_queueimpl_42 {
() => {
// Module: crate::mpsc::queue
// Provides: {"impl_42"}
// Dependencies: {}
impl < T > Node < T > { unsafe fn new (v : Option < T >) -> * mut Self { Box :: into_raw (Box :: new (Self { next : AtomicPtr :: new (ptr :: null_mut ()) , value : v })) } }
};
}
