// Generated macro for impl_98 (impl)
macro_rules! Depcrate_address_queueimpl_98 {
() => {
// Module: crate::address::queue
// Provides: {"impl_98"}
// Dependencies: {}
impl < T > Node < T > { unsafe fn new (v : Option < T >) -> * mut Self { Box :: into_raw (Box :: new (Self { next : AtomicPtr :: new (ptr :: null_mut ()) , value : v , })) } }
};
}
