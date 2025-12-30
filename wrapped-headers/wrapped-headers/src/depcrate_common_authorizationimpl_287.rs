// Generated macro for impl_287 (impl)
macro_rules! Depcrate_common_authorizationimpl_287 {
() => {
// Module: crate::common::authorization
// Provides: {"impl_287"}
// Dependencies: {}
impl Bearer { # [doc = " View the token part as a `&str`."] pub fn token (& self) -> & str { self . 0 . as_str () ["Bearer " . len () ..] . trim_start () } }
};
}
