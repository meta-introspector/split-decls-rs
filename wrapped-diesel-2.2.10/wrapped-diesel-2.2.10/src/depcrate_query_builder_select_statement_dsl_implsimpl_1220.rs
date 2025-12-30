// Generated macro for impl_1220 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1220 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1220"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , Rhs , Kind , On > InternalJoinDsl < Rhs , Kind , On > for SelectStatement < FromClause < F > , SelectClause < S > , D , W , O , LOf , G , H , LC > where F : QuerySource , Rhs : QuerySource , JoinOn < Join < F , Rhs , Kind > , On > : QuerySource , SelectStatement < FromClause < JoinOn < Join < F , Rhs , Kind > , On > > , SelectClause < S > , D , W , O , LOf , G , H , LC , > : AsQuery , { type Output = SelectStatement < FromClause < JoinOn < Join < F , Rhs , Kind > , On > > , SelectClause < S > , D , W , O , LOf , G , H , LC , > ; fn join (self , rhs : Rhs , kind : Kind , on : On) -> Self :: Output { SelectStatement :: new (self . select , FromClause :: new (Join :: new (self . from . source , rhs , kind) . on (on)) , self . distinct , self . where_clause , self . order , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
