// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl Acquired { # [doc = " This drops the [`Acquired`] token without releasing the associated token."] # [doc = ""] # [doc = " This is not generally useful, but can be helpful if you do not have the"] # [doc = " ability to store an Acquired token but need to not yet release it."] # [doc = ""] # [doc = " You'll typically want to follow this up with a call to"] # [doc = " [`Client::release_raw`] or similar to actually release the token later on."] pub fn drop_without_releasing (mut self) { self . disabled = true ; } }
};
}
