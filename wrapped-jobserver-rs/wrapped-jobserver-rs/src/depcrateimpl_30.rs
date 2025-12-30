// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl HelperThread { # [doc = " Request that the helper thread acquires a token, eventually calling the"] # [doc = " original closure with a token when it's available."] # [doc = ""] # [doc = " For more information, see the docs on [`Client::into_helper_thread`]."] pub fn request_token (& self) { self . state . lock () . requests += 1 ; self . state . cvar . notify_one () ; } }
};
}
