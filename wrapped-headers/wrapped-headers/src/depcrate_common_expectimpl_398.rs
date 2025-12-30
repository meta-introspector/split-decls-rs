// Generated macro for impl_398 (impl)
macro_rules! Depcrate_common_expectimpl_398 {
() => {
// Module: crate::common::expect
// Provides: {"impl_398"}
// Dependencies: {}
impl Header for Expect { fn name () -> & 'static HeaderName { & :: http :: header :: EXPECT } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { values . just_one () . and_then (| value | { if value == "100-continue" { Some (Expect :: CONTINUE) } else { None } }) . ok_or_else (Error :: invalid) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { values . extend (:: std :: iter :: once (HeaderValue :: from_static ("100-continue"))) ; } }
};
}
