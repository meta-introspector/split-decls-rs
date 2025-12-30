// Generated macro for payload_as_str (function)
macro_rules! Depcrate_panic_hookpayload_as_str {
() => {
// Module: crate::panic_hook
// Provides: {"payload_as_str"}
// Dependencies: {}
# [doc = " FIXME(Zalathar): Replace with `PanicHookInfo::payload_as_str` when that's"] # [doc = " stable in beta."] fn payload_as_str < 'a > (info : & 'a PanicHookInfo < '_ >) -> Option < & 'a str > { let payload = info . payload () ; if let Some (s) = payload . downcast_ref :: < & str > () { Some (s) } else if let Some (s) = payload . downcast_ref :: < String > () { Some (s) } else { None } }
};
}
