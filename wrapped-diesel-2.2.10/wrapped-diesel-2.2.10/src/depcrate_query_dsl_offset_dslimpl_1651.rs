// Generated macro for impl_1651 (impl)
macro_rules! Depcrate_query_dsl_offset_dslimpl_1651 {
() => {
// Module: crate::query_dsl::offset_dsl
// Provides: {"impl_1651"}
// Dependencies: {}
impl < T > OffsetDsl for T where T : Table , T :: Query : OffsetDsl , { type Output = < T :: Query as OffsetDsl > :: Output ; fn offset (self , offset : i64) -> Self :: Output { self . as_query () . offset (offset) } }
};
}
