// Generated macro for impl_23 (impl)
macro_rules! Depcrate_eventsource_futuresimpl_23 {
() => {
// Module: crate::eventsource::futures
// Provides: {"impl_23"}
// Dependencies: {}
impl fmt :: Debug for EventSource { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("EventSource") . field ("url" , & self . es . url ()) . field ("with_credentials" , & self . es . with_credentials ()) . field ("ready_state" , & self . state ()) . finish_non_exhaustive () } }
};
}
