// Generated macro for impl_607 (impl)
macro_rules! Depcrate_common_strict_transport_securityimpl_607 {
() => {
// Module: crate::common::strict_transport_security
// Provides: {"impl_607"}
// Dependencies: {}
impl Header for StrictTransportSecurity { fn name () -> & 'static HeaderName { & :: http :: header :: STRICT_TRANSPORT_SECURITY } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { values . just_one () . and_then (| v | v . to_str () . ok ()) . map (from_str) . unwrap_or_else (| | Err (Error :: invalid ())) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { struct Adapter < 'a > (& 'a StrictTransportSecurity) ; impl fmt :: Display for Adapter < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . 0 . include_subdomains { write ! (f , "max-age={}; includeSubdomains" , self . 0 . max_age) } else { write ! (f , "max-age={}" , self . 0 . max_age) } } } values . extend (:: std :: iter :: once (util :: fmt (Adapter (self)))) ; } }
};
}
