// Generated macro for impl_25 (impl)
macro_rules! Depcrate_eventsource_futuresimpl_25 {
() => {
// Module: crate::eventsource::futures
// Provides: {"impl_25"}
// Dependencies: {}
impl fmt :: Debug for EventSourceSubscription { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("EventSourceSubscription") . field ("event_source" , & self . es) . field ("event_type" , & self . event_type) . finish_non_exhaustive () } }
};
}
