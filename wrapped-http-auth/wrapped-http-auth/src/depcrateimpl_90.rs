// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
# [doc = " Tries to create a `PasswordClient` from the supplied `str` challenge list."] # [doc = ""] # [doc = " This is a convenience wrapper around [`PasswordClientBuilder`]."] impl TryFrom < & str > for PasswordClient { type Error = String ; # [inline] fn try_from (value : & str) -> Result < Self , Self :: Error > { PasswordClient :: builder () . challenges (value) . build () } }
};
}
