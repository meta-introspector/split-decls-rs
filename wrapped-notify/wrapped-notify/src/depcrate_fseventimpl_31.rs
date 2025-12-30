// Generated macro for impl_31 (impl)
macro_rules! Depcrate_fseventimpl_31 {
() => {
// Module: crate::fsevent
// Provides: {"impl_31"}
// Dependencies: {}
impl fmt :: Debug for FsEventWatcher { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("FsEventWatcher") . field ("paths" , & self . paths) . field ("since_when" , & self . since_when) . field ("latency" , & self . latency) . field ("flags" , & self . flags) . field ("event_handler" , & Arc :: as_ptr (& self . event_handler)) . field ("runloop" , & self . runloop) . field ("recursive_info" , & self . recursive_info) . finish () } }
};
}
