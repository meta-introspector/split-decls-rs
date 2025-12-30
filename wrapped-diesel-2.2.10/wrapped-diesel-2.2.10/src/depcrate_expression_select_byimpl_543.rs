// Generated macro for impl_543 (impl)
macro_rules! Depcrate_expression_select_byimpl_543 {
() => {
// Module: crate::expression::select_by
// Provides: {"impl_543"}
// Dependencies: {}
impl < T , E , DB > QueryId for SelectBy < T , DB > where DB : Backend , T : Selectable < DB , SelectExpression = E > , E : QueryId + Expression , { type QueryId = E :: QueryId ; const HAS_STATIC_QUERY_ID : bool = E :: HAS_STATIC_QUERY_ID ; }
};
}
