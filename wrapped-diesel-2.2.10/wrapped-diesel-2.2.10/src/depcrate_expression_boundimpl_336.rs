// Generated macro for impl_336 (impl)
macro_rules! Depcrate_expression_boundimpl_336 {
() => {
// Module: crate::expression::bound
// Provides: {"impl_336"}
// Dependencies: {}
impl < T : QueryId , U > QueryId for Bound < T , U > { type QueryId = Bound < T :: QueryId , () > ; const HAS_STATIC_QUERY_ID : bool = T :: HAS_STATIC_QUERY_ID ; }
};
}
