// Generated macro for impl_14 (impl)
macro_rules! Depcrate_main_loopimpl_14 {
() => {
// Module: crate::main_loop
// Provides: {"impl_14"}
// Dependencies: {}
impl SpanTransformer for SpanTrans { type Table = () ; type Span = SpanId ; fn token_id_of (_ : & mut Self :: Table , span : Self :: Span ,) -> proc_macro_api :: legacy_protocol :: SpanId { proc_macro_api :: legacy_protocol :: SpanId (span . 0) } fn span_for_token_id (_ : & Self :: Table , id : proc_macro_api :: legacy_protocol :: SpanId ,) -> Self :: Span { SpanId (id . 0) } }
};
}
