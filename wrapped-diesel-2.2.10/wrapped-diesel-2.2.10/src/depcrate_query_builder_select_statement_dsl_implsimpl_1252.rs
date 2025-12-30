// Generated macro for impl_1252 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1252 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1252"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , Predicate > HavingDsl < Predicate > for SelectStatement < FromClause < F > , S , D , W , O , LOf , GroupByClause < G > , H > where F : QuerySource , Predicate : AppearsOnTable < F > , Predicate : Expression , Predicate :: SqlType : BoolOrNullableBool , { type Output = SelectStatement < FromClause < F > , S , D , W , O , LOf , GroupByClause < G > , HavingClause < Predicate > > ; fn having (self , predicate : Predicate) -> Self :: Output { SelectStatement :: new (self . select , self . from , self . distinct , self . where_clause , self . order , self . limit_offset , self . group_by , HavingClause (predicate) , self . locking ,) } }
};
}
