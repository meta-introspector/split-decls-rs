// Generated macro for impl_726 (impl)
macro_rules! Depcrate_frame_settingsimpl_726 {
() => {
// Module: crate::frame::settings
// Provides: {"impl_726"}
// Dependencies: {}
impl fmt :: Debug for Settings { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut builder = f . debug_struct ("Settings") ; builder . field ("flags" , & self . flags) ; self . for_each (| setting | match setting { Setting :: EnablePush (v) => { builder . field ("enable_push" , & v) ; } Setting :: HeaderTableSize (v) => { builder . field ("header_table_size" , & v) ; } Setting :: InitialWindowSize (v) => { builder . field ("initial_window_size" , & v) ; } Setting :: MaxConcurrentStreams (v) => { builder . field ("max_concurrent_streams" , & v) ; } Setting :: MaxFrameSize (v) => { builder . field ("max_frame_size" , & v) ; } Setting :: MaxHeaderListSize (v) => { builder . field ("max_header_list_size" , & v) ; } Setting :: EnableConnectProtocol (v) => { builder . field ("enable_connect_protocol" , & v) ; } }) ; builder . finish () } }
};
}
