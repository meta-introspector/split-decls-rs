// Generated macro for impl_1798 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1798 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1798"}
// Dependencies: {}
impl < S , Predicate > OrFilterDsl < Predicate > for Alias < S > where Self : AsQuery , < Self as AsQuery > :: Query : OrFilterDsl < Predicate > , { type Output = dsl :: OrFilter < < Self as AsQuery > :: Query , Predicate > ; fn or_filter (self , predicate : Predicate) -> Self :: Output { self . as_query () . or_filter (predicate) } }
};
}
