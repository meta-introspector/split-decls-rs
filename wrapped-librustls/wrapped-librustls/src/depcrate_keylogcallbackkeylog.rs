// Generated macro for CallbackKeyLog (struct)
macro_rules! Depcrate_keylogCallbackKeyLog {
() => {
// Module: crate::keylog
// Provides: {"CallbackKeyLog"}
// Dependencies: {}
# [doc = " An implementation of `rustls::KeyLog` based on C callbacks."] pub (crate) struct CallbackKeyLog { pub (crate) log_cb : KeylogLogCallback , pub (crate) will_log_cb : rustls_keylog_will_log_callback , }
};
}
