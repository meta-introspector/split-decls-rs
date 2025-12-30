// Generated macro for impl_527 (impl)
macro_rules! Depcrate_common_refererimpl_527 {
() => {
// Module: crate::common::referer
// Provides: {"impl_527"}
// Dependencies: {}
impl Referer { # [doc = " Create a `Referer` with a static string."] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " Panics if the string is not a legal header value."] pub const fn from_static (s : & 'static str) -> Referer { Referer (HeaderValueString :: from_static (s)) } }
};
}
