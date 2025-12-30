// Generated macro for impl_94 (impl)
macro_rules! Depcrateimpl_94 {
() => {
// Module: crate
// Provides: {"impl_94"}
// Dependencies: {}
# [doc = " Tries to create a `PasswordClient` from the supplied `http::header::GetAll` challenge lists."] # [doc = ""] # [doc = " This is a convenience wrapper around [`PasswordClientBuilder`]."] # [cfg (feature = "http10")] # [cfg_attr (docsrs , doc (cfg (feature = "http10")))] impl TryFrom < http10 :: header :: GetAll < '_ , http10 :: HeaderValue > > for PasswordClient { type Error = String ; fn try_from (value : http10 :: header :: GetAll < '_ , http10 :: HeaderValue > ,) -> Result < Self , Self :: Error > { let mut builder = PasswordClient :: builder () ; for v in value { builder = builder . header_value (v) ; } builder . build () } }
};
}
