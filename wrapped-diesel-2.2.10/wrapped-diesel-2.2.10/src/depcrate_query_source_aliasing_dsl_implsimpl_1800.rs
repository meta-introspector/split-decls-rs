// Generated macro for impl_1800 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1800 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1800"}
// Dependencies: {}
impl < S > LimitDsl for Alias < S > where Self : AsQuery , < Self as AsQuery > :: Query : LimitDsl , { type Output = < < Self as AsQuery > :: Query as LimitDsl > :: Output ; fn limit (self , limit : i64) -> Self :: Output { self . as_query () . limit (limit) } }
};
}
