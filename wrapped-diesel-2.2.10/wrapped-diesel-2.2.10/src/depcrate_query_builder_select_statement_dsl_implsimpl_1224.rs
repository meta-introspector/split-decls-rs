// Generated macro for impl_1224 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1224 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1224"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , Predicate > FilterDsl < Predicate > for SelectStatement < F , S , D , W , O , LOf , G , H , LC > where Predicate : Expression + NonAggregate , Predicate :: SqlType : BoolOrNullableBool , W : WhereAnd < Predicate > , { type Output = SelectStatement < F , S , D , W :: Output , O , LOf , G , H , LC > ; fn filter (self , predicate : Predicate) -> Self :: Output { SelectStatement :: new (self . select , self . from , self . distinct , self . where_clause . and (predicate) , self . order , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
