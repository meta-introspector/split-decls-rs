// Generated macro for impl_633 (impl)
macro_rules! Depcrate_common_user_agentimpl_633 {
() => {
// Module: crate::common::user_agent
// Provides: {"impl_633"}
// Dependencies: {}
impl UserAgent { # [doc = " Create a `UserAgent` from a static string."] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " Panics if the static string is not a legal header value."] pub const fn from_static (src : & 'static str) -> UserAgent { UserAgent (HeaderValueString :: from_static (src)) } # [doc = " View this `UserAgent` as a `&str`."] pub fn as_str (& self) -> & str { self . 0 . as_str () } }
};
}
