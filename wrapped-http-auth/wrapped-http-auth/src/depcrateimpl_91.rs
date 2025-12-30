// Generated macro for impl_91 (impl)
macro_rules! Depcrateimpl_91 {
() => {
// Module: crate
// Provides: {"impl_91"}
// Dependencies: {}
# [doc = " Tries to create a `PasswordClient` from the supplied `HeaderValue` challenge list."] # [doc = ""] # [doc = " This is a convenience wrapper around [`PasswordClientBuilder`]."] # [cfg (feature = "http")] # [cfg_attr (docsrs , doc (cfg (feature = "http")))] impl TryFrom < & http :: HeaderValue > for PasswordClient { type Error = String ; # [inline] fn try_from (value : & http :: HeaderValue) -> Result < Self , Self :: Error > { PasswordClient :: builder () . header_value (value) . build () } }
};
}
