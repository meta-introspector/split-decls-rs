// Generated macro for impl_1240 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1240 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1240"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , LM , Modifier > ModifyLockDsl < Modifier > for SelectStatement < F , S , D , W , O , LOf , G , H , LockingClause < LC , LM > > { type Output = SelectStatement < F , S , D , W , O , LOf , G , H , LockingClause < LC , Modifier > > ; fn modify_lock (self , modifier : Modifier) -> Self :: Output { SelectStatement :: new (self . select , self . from , self . distinct , self . where_clause , self . order , self . limit_offset , self . group_by , self . having , LockingClause :: new (self . locking . lock_mode , modifier) ,) } }
};
}
