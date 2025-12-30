// Generated macro for impl_1588 (impl)
macro_rules! Depcrate_query_dsl_filter_dslimpl_1588 {
() => {
// Module: crate::query_dsl::filter_dsl
// Provides: {"impl_1588"}
// Dependencies: {}
impl < T , Predicate > OrFilterDsl < Predicate > for T where T : Table , T :: Query : OrFilterDsl < Predicate > , { type Output = OrFilter < T :: Query , Predicate > ; fn or_filter (self , predicate : Predicate) -> Self :: Output { self . as_query () . or_filter (predicate) } }
};
}
