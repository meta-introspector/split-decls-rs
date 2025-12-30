// Generated macro for impl_3487 (impl)
macro_rules! Depcrate_pg_query_builder_distinct_onimpl_3487 {
() => {
// Module: crate::pg::query_builder::distinct_on
// Provides: {"impl_3487"}
// Dependencies: {}
impl < ST , F , S , D , W , O , LOf , G , H , Selection > DistinctOnDsl < Selection > for SelectStatement < FromClause < F > , S , D , W , O , LOf , G , H > where F : QuerySource , Selection : SelectableExpression < F > , Self : SelectQuery < SqlType = ST > , O : ValidOrderingForDistinct < DistinctOnClause < Selection > > , SelectStatement < FromClause < F > , S , DistinctOnClause < Selection > , W , O , LOf , G , H > : SelectQuery < SqlType = ST > , { type Output = SelectStatement < FromClause < F > , S , DistinctOnClause < Selection > , W , O , LOf , G , H > ; fn distinct_on (self , selection : Selection) -> Self :: Output { SelectStatement :: new (self . select , self . from , DistinctOnClause (selection) , self . where_clause , self . order , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
