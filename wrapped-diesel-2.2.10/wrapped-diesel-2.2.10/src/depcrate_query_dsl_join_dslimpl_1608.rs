// Generated macro for impl_1608 (impl)
macro_rules! Depcrate_query_dsl_join_dslimpl_1608 {
() => {
// Module: crate::query_dsl::join_dsl
// Provides: {"impl_1608"}
// Dependencies: {}
impl < T , Rhs , Kind , On > InternalJoinDsl < Rhs , Kind , On > for T where T : Table + AsQuery , T :: Query : InternalJoinDsl < Rhs , Kind , On > , { type Output = < T :: Query as InternalJoinDsl < Rhs , Kind , On > > :: Output ; fn join (self , rhs : Rhs , kind : Kind , on : On) -> Self :: Output { self . as_query () . join (rhs , kind , on) } }
};
}
