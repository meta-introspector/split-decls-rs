// Generated macro for impl_51 (impl)
macro_rules! Depcrate_hashimpl_51 {
() => {
// Module: crate::hash
// Provides: {"impl_51"}
// Dependencies: {}
impl HashHistory { # [doc = " Creates a new [`HashHistory`]"] pub fn new () -> Self { Self :: default () } fn get_url () -> Url { let href = window () . location () . href () . expect_throw ("Failed to read location href") ; Url :: new (& href) . expect_throw ("current url is not valid.") } }
};
}
