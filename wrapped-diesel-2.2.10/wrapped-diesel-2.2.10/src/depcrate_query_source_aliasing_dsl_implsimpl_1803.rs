// Generated macro for impl_1803 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1803 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1803"}
// Dependencies: {}
impl < S > OffsetDsl for Alias < S > where Self : AsQuery , < Self as AsQuery > :: Query : OffsetDsl , { type Output = < < Self as AsQuery > :: Query as OffsetDsl > :: Output ; fn offset (self , offset : i64) -> Self :: Output { self . as_query () . offset (offset) } }
};
}
