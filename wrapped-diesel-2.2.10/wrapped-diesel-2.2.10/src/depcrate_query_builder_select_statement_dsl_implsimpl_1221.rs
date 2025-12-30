// Generated macro for impl_1221 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1221 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1221"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , Selection > SelectDsl < Selection > for SelectStatement < FromClause < F > , S , D , W , O , LOf , G , H , LC > where G : ValidGroupByClause , F : QuerySource , Selection : SelectableExpression < F > + ValidGrouping < G :: Expressions > , SelectStatement < FromClause < F > , SelectClause < Selection > , D , W , O , LOf , G , H , LC > : SelectQuery , { type Output = SelectStatement < FromClause < F > , SelectClause < Selection > , D , W , O , LOf , G , H , LC > ; fn select (self , selection : Selection) -> Self :: Output { SelectStatement :: new (SelectClause (selection) , self . from , self . distinct , self . where_clause , self . order , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
