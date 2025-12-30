// Generated macro for impl_1237 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1237 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1237"}
// Dependencies: {}
impl < ST , F , S , D , W , O , L , Of , G , H , LC > OffsetDsl for SelectStatement < F , S , D , W , O , LimitOffsetClause < L , Of > , G , H , LC > where Self : SelectQuery < SqlType = ST > , SelectStatement < F , S , D , W , O , LimitOffsetClause < L , OffsetClause < Offset > > , G , H , LC > : SelectQuery < SqlType = ST > , { type Output = SelectStatement < F , S , D , W , O , LimitOffsetClause < L , OffsetClause < Offset > > , G , H , LC > ; fn offset (self , offset : i64) -> Self :: Output { let offset_clause = OffsetClause (offset . into_sql :: < BigInt > ()) ; SelectStatement :: new (self . select , self . from , self . distinct , self . where_clause , self . order , LimitOffsetClause { limit_clause : self . limit_offset . limit_clause , offset_clause , } , self . group_by , self . having , self . locking ,) } }
};
}
