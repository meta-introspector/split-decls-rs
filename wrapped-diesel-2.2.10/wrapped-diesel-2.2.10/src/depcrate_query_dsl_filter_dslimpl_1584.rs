// Generated macro for impl_1584 (impl)
macro_rules! Depcrate_query_dsl_filter_dslimpl_1584 {
() => {
// Module: crate::query_dsl::filter_dsl
// Provides: {"impl_1584"}
// Dependencies: {}
impl < T , Predicate > FilterDsl < Predicate > for T where T : Table , T :: Query : FilterDsl < Predicate > , { type Output = Filter < T :: Query , Predicate > ; fn filter (self , predicate : Predicate) -> Self :: Output { self . as_query () . filter (predicate) } }
};
}
