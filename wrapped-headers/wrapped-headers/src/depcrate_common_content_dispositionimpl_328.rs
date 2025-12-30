// Generated macro for impl_328 (impl)
macro_rules! Depcrate_common_content_dispositionimpl_328 {
() => {
// Module: crate::common::content_disposition
// Provides: {"impl_328"}
// Dependencies: {}
impl Header for ContentDisposition { fn name () -> & 'static HeaderName { & :: http :: header :: CONTENT_DISPOSITION } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { values . next () . cloned () . map (ContentDisposition) . ok_or_else (Error :: invalid) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { values . extend (:: std :: iter :: once (self . 0 . clone ())) ; } }
};
}
