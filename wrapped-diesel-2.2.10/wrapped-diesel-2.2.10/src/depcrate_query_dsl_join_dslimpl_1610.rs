// Generated macro for impl_1610 (impl)
macro_rules! Depcrate_query_dsl_join_dslimpl_1610 {
() => {
// Module: crate::query_dsl::join_dsl
// Provides: {"impl_1610"}
// Dependencies: {}
impl < Lhs , Rhs , Kind > JoinWithImplicitOnClause < Rhs , Kind > for Lhs where Lhs : JoinTo < Rhs > , Lhs : InternalJoinDsl < < Lhs as JoinTo < Rhs > > :: FromClause , Kind , < Lhs as JoinTo < Rhs > > :: OnClause > , { type Output = < Lhs as InternalJoinDsl < Lhs :: FromClause , Kind , Lhs :: OnClause > > :: Output ; fn join_with_implicit_on_clause (self , rhs : Rhs , kind : Kind) -> Self :: Output { let (from , on) = Lhs :: join_target (rhs) ; self . join (from , kind , on) } }
};
}
