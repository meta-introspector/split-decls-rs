// Generated macro for impl_1792 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1792 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1792"}
// Dependencies: {}
impl < S , Predicate > FilterDsl < Predicate > for Alias < S > where Self : AsQuery , < Self as AsQuery > :: Query : FilterDsl < Predicate > , { type Output = dsl :: Filter < < Self as AsQuery > :: Query , Predicate > ; fn filter (self , predicate : Predicate) -> Self :: Output { self . as_query () . filter (predicate) } }
};
}
