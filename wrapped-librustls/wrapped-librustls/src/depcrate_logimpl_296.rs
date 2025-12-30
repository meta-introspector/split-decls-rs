// Generated macro for impl_296 (impl)
macro_rules! Depcrate_logimpl_296 {
() => {
// Module: crate::log
// Provides: {"impl_296"}
// Dependencies: {}
# [cfg (not (feature = "no_log_capture"))] impl log :: Log for Logger { fn enabled (& self , _metadata : & log :: Metadata < '_ >) -> bool { true } fn log (& self , record : & log :: Record < '_ >) { if let Ok ((Some (cb) , userdata)) = log_callback_get () { let message = format ! ("{} {}" , record . target () , record . args ()) ; if let Ok (message) = message . as_str () . try_into () { unsafe { cb (userdata , & rustls_log_params { level : record . level () as rustls_log_level , message , } ,) ; } } } } fn flush (& self) { } }
};
}
