// Generated macro for InternalJoinDsl (trait)
macro_rules! Depcrate_query_dsl_join_dslInternalJoinDsl {
() => {
// Module: crate::query_dsl::join_dsl
// Provides: {"InternalJoinDsl"}
// Dependencies: {}
# [doc (hidden)] # [doc = " `JoinDsl` support trait to emulate associated type constructors"] pub trait InternalJoinDsl < Rhs , Kind , On > { type Output ; fn join (self , rhs : Rhs , kind : Kind , on : On) -> Self :: Output ; }
};
}
