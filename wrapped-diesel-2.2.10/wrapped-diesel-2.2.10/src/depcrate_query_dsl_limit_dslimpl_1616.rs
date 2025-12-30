// Generated macro for impl_1616 (impl)
macro_rules! Depcrate_query_dsl_limit_dslimpl_1616 {
() => {
// Module: crate::query_dsl::limit_dsl
// Provides: {"impl_1616"}
// Dependencies: {}
impl < T > LimitDsl for T where T : Table , T :: Query : LimitDsl , { type Output = < T :: Query as LimitDsl > :: Output ; fn limit (self , limit : i64) -> Self :: Output { self . as_query () . limit (limit) } }
};
}
