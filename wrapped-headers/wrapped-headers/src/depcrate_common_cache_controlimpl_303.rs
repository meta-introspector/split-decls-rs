// Generated macro for impl_303 (impl)
macro_rules! Depcrate_common_cache_controlimpl_303 {
() => {
// Module: crate::common::cache_control
// Provides: {"impl_303"}
// Dependencies: {}
impl Header for CacheControl { fn name () -> & 'static HeaderName { & :: http :: header :: CACHE_CONTROL } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { csv :: from_comma_delimited (values) . map (| FromIter (cc) | cc) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { values . extend (:: std :: iter :: once (util :: fmt (Fmt (self)))) ; } }
};
}
