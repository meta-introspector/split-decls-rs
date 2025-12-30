// Generated macro for JoinWithImplicitOnClause (trait)
macro_rules! Depcrate_query_dsl_join_dslJoinWithImplicitOnClause {
() => {
// Module: crate::query_dsl::join_dsl
// Provides: {"JoinWithImplicitOnClause"}
// Dependencies: {}
# [doc (hidden)] # [doc = " `JoinDsl` support trait to emulate associated type constructors and grab"] # [doc = " the known on clause from the associations API"] pub trait JoinWithImplicitOnClause < Rhs , Kind > { type Output ; fn join_with_implicit_on_clause (self , rhs : Rhs , kind : Kind) -> Self :: Output ; }
};
}
