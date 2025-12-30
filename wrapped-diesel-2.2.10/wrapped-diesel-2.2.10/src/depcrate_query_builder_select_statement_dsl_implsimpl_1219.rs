// Generated macro for impl_1219 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1219 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1219"}
// Dependencies: {}
impl < F , D , W , O , LOf , G , H , LC , Rhs , Kind , On > InternalJoinDsl < Rhs , Kind , On > for SelectStatement < FromClause < F > , DefaultSelectClause < FromClause < F > > , D , W , O , LOf , G , H , LC > where F : QuerySource , Rhs : QuerySource , JoinOn < Join < F , Rhs , Kind > , On > : QuerySource , SelectStatement < FromClause < JoinOn < Join < F , Rhs , Kind > , On > > , DefaultSelectClause < FromClause < JoinOn < Join < F , Rhs , Kind > , On > > > , D , W , O , LOf , G , H , LC , > : AsQuery , { type Output = SelectStatement < FromClause < JoinOn < Join < F , Rhs , Kind > , On > > , DefaultSelectClause < FromClause < JoinOn < Join < F , Rhs , Kind > , On > > > , D , W , O , LOf , G , H , LC , > ; fn join (self , rhs : Rhs , kind : Kind , on : On) -> Self :: Output { let from = FromClause :: new (Join :: new (self . from . source , rhs , kind) . on (on)) ; SelectStatement :: new (DefaultSelectClause :: new (& from) , from , self . distinct , self . where_clause , self . order , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
