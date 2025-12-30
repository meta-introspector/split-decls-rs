// Generated macro for impl_284 (impl)
macro_rules! Depcrate_common_authorizationimpl_284 {
() => {
// Module: crate::common::authorization
// Provides: {"impl_284"}
// Dependencies: {}
impl Basic { # [doc = " View the decoded username."] pub fn username (& self) -> & str { & self . decoded [.. self . colon_pos] } # [doc = " View the decoded password."] pub fn password (& self) -> & str { & self . decoded [self . colon_pos + 1 ..] } }
};
}
