// Generated macro for impl_1239 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1239 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1239"}
// Dependencies: {}
impl < F , S , W , O , LOf , Lock > LockingDsl < Lock > for SelectStatement < F , S , NoDistinctClause , W , O , LOf > { type Output = SelectStatement < F , S , NoDistinctClause , W , O , LOf , NoGroupByClause , NoHavingClause , LockingClause < Lock , NoModifier > , > ; fn with_lock (self , lock : Lock) -> Self :: Output { SelectStatement :: new (self . select , self . from , self . distinct , self . where_clause , self . order , self . limit_offset , self . group_by , self . having , LockingClause :: new (lock , NoModifier) ,) } }
};
}
