// Generated macro for impl_279 (impl)
macro_rules! Depcrate_common_authorizationimpl_279 {
() => {
// Module: crate::common::authorization
// Provides: {"impl_279"}
// Dependencies: {}
impl Authorization < Basic > { # [doc = " Create a `Basic` authorization header."] pub fn basic (username : & str , password : & str) -> Self { let colon_pos = username . len () ; let decoded = format ! ("{}:{}" , username , password) ; Authorization (Basic { decoded , colon_pos }) } # [doc = " View the decoded username."] pub fn username (& self) -> & str { self . 0 . username () } # [doc = " View the decoded password."] pub fn password (& self) -> & str { self . 0 . password () } }
};
}
