// Generated macro for config_log (function)
macro_rules! Depcrate_traceconfig_log {
() => {
// Module: crate::trace
// Provides: {"config_log"}
// Dependencies: {}
# [doc = " Set up the process-wide SQLite error logging callback."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is marked unsafe for two reasons:"] # [doc = ""] # [doc = " * The function is not threadsafe. No other SQLite calls may be made while"] # [doc = "   `config_log` is running, and multiple threads may not call `config_log`"] # [doc = "   simultaneously."] # [doc = " * The provided `callback` itself function has two requirements:"] # [doc = "     * It must not invoke any SQLite calls."] # [doc = "     * It must be threadsafe if SQLite is used in a multithreaded way."] # [doc = ""] # [doc = " cf [The Error And Warning Log](http://sqlite.org/errlog.html)."] # [cfg (not (feature = "loadable_extension"))] pub unsafe fn config_log (callback : Option < fn (c_int , & str) >) -> crate :: Result < () > { extern "C" fn log_callback (p_arg : * mut c_void , err : c_int , msg : * const c_char) { let s = unsafe { CStr :: from_ptr (msg) . to_string_lossy () } ; let callback : fn (c_int , & str) = unsafe { mem :: transmute (p_arg) } ; drop (catch_unwind (| | callback (err , & s))) ; } let rc = if let Some (f) = callback { ffi :: sqlite3_config (ffi :: SQLITE_CONFIG_LOG , log_callback as extern "C" fn (_ , _ , _) , f as * mut c_void ,) } else { let nullptr : * mut c_void = ptr :: null_mut () ; ffi :: sqlite3_config (ffi :: SQLITE_CONFIG_LOG , nullptr , nullptr) } ; if rc == ffi :: SQLITE_OK { Ok (()) } else { Err (crate :: error :: error_from_sqlite_code (rc , None)) } }
};
}
