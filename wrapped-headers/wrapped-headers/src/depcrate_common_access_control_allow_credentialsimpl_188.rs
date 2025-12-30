// Generated macro for impl_188 (impl)
macro_rules! Depcrate_common_access_control_allow_credentialsimpl_188 {
() => {
// Module: crate::common::access_control_allow_credentials
// Provides: {"impl_188"}
// Dependencies: {}
impl Header for AccessControlAllowCredentials { fn name () -> & 'static HeaderName { & :: http :: header :: ACCESS_CONTROL_ALLOW_CREDENTIALS } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { values . next () . and_then (| value | { if value == "true" { Some (AccessControlAllowCredentials) } else { None } }) . ok_or_else (Error :: invalid) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { values . extend (:: std :: iter :: once (HeaderValue :: from_static ("true"))) ; } }
};
}
