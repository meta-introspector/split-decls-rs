// Generated macro for impl_281 (impl)
macro_rules! Depcrate_common_authorizationimpl_281 {
() => {
// Module: crate::common::authorization
// Provides: {"impl_281"}
// Dependencies: {}
impl < C : Credentials > Header for Authorization < C > { fn name () -> & 'static HeaderName { & :: http :: header :: AUTHORIZATION } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { values . next () . and_then (| val | { let slice = val . as_bytes () ; if slice . len () > C :: SCHEME . len () && slice [C :: SCHEME . len ()] == b' ' && slice [.. C :: SCHEME . len ()] . eq_ignore_ascii_case (C :: SCHEME . as_bytes ()) { C :: decode (val) . map (Authorization) } else { None } }) . ok_or_else (Error :: invalid) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { let mut value = self . 0 . encode () ; value . set_sensitive (true) ; debug_assert ! (value . as_bytes () . starts_with (C :: SCHEME . as_bytes ()) , "Credentials::encode should include its scheme: scheme = {:?}, encoded = {:?}" , C :: SCHEME , value ,) ; values . extend (:: std :: iter :: once (value)) ; } }
};
}
