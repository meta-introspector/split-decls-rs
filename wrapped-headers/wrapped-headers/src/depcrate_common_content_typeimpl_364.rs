// Generated macro for impl_364 (impl)
macro_rules! Depcrate_common_content_typeimpl_364 {
() => {
// Module: crate::common::content_type
// Provides: {"impl_364"}
// Dependencies: {}
impl Header for ContentType { fn name () -> & 'static HeaderName { & :: http :: header :: CONTENT_TYPE } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { values . next () . and_then (| v | v . to_str () . ok () ? . parse () . ok ()) . map (ContentType) . ok_or_else (Error :: invalid) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { let value = self . 0 . as_ref () . parse () . expect ("Mime is always a valid HeaderValue") ; values . extend (:: std :: iter :: once (value)) ; } }
};
}
