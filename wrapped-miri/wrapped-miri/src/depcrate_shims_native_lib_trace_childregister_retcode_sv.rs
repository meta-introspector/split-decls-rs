// Generated macro for register_retcode_sv (function)
macro_rules! Depcrate_shims_native_lib_trace_childregister_retcode_sv {
() => {
// Module: crate::shims::native_lib::trace::child
// Provides: {"register_retcode_sv"}
// Dependencies: {}
# [doc = " Instruct the supervisor process to return a particular code. Useful if for"] # [doc = " whatever reason this code fails to be intercepted normally."] pub fn register_retcode_sv (code : i32) { let mut sv_guard = SUPERVISOR . lock () . unwrap () ; if let Some (sv) = sv_guard . as_mut () { sv . message_tx . send (TraceRequest :: OverrideRetcode (code)) . unwrap () ; sv . confirm_rx . recv () . unwrap () ; } }
};
}
