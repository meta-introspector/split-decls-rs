// Generated macro for impl_587 (impl)
macro_rules! Depcrate_common_serverimpl_587 {
() => {
// Module: crate::common::server
// Provides: {"impl_587"}
// Dependencies: {}
impl Server { # [doc = " Construct a `Server` from a static string."] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " Panics if the static string is not a legal header value."] pub const fn from_static (s : & 'static str) -> Server { Server (HeaderValueString :: from_static (s)) } # [doc = " View this `Server` as a `&str`."] pub fn as_str (& self) -> & str { self . 0 . as_str () } }
};
}
