// Generated macro for foreach_cb (function)
macro_rules! Depcrate_odbforeach_cb {
() => {
// Module: crate::odb
// Provides: {"foreach_cb"}
// Dependencies: {}
extern "C" fn foreach_cb (id : * const raw :: git_oid , payload : * mut c_void) -> c_int { panic :: wrap (| | unsafe { let data = & mut * (payload as * mut ForeachCbData < '_ >) ; let res = { let callback = & mut data . callback ; callback (& Binding :: from_raw (id)) } ; if res { 0 } else { 1 } }) . unwrap_or (1) }
};
}
