// Generated macro for impl_1222 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1222 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1222"}
// Dependencies: {}
impl < S , D , W , O , LOf , G , H , LC , Selection > SelectDsl < Selection > for SelectStatement < NoFromClause , S , D , W , O , LOf , G , H , LC > where G : ValidGroupByClause , Selection : SelectableExpression < NoFromClause > + ValidGrouping < G :: Expressions > , SelectStatement < NoFromClause , SelectClause < Selection > , D , W , O , LOf , G , H , LC > : SelectQuery , { type Output = SelectStatement < NoFromClause , SelectClause < Selection > , D , W , O , LOf , G , H , LC > ; fn select (self , selection : Selection) -> Self :: Output { SelectStatement :: new (SelectClause (selection) , self . from , self . distinct , self . where_clause , self . order , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
