// Generated macro for impl_1235 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1235 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1235"}
// Dependencies: {}
impl < ST , F , S , D , W , O , L , Of , G , H , LC > LimitDsl for SelectStatement < F , S , D , W , O , LimitOffsetClause < L , Of > , G , H , LC > where Self : SelectQuery < SqlType = ST > , SelectStatement < F , S , D , W , O , LimitOffsetClause < LimitClause < Limit > , Of > , G , H , LC > : SelectQuery < SqlType = ST > , { type Output = SelectStatement < F , S , D , W , O , LimitOffsetClause < LimitClause < Limit > , Of > , G , H , LC > ; fn limit (self , limit : i64) -> Self :: Output { let limit_clause = LimitClause (limit . into_sql :: < BigInt > ()) ; SelectStatement :: new (self . select , self . from , self . distinct , self . where_clause , self . order , LimitOffsetClause { limit_clause , offset_clause : self . limit_offset . offset_clause , } , self . group_by , self . having , self . locking ,) } }
};
}
