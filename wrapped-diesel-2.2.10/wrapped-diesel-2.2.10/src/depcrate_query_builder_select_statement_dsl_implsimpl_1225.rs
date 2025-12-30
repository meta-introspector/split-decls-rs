// Generated macro for impl_1225 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1225 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1225"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , Predicate > OrFilterDsl < Predicate > for SelectStatement < F , S , D , W , O , LOf , G , H , LC > where Predicate : Expression + NonAggregate , Predicate :: SqlType : BoolOrNullableBool , W : WhereOr < Predicate > , { type Output = SelectStatement < F , S , D , W :: Output , O , LOf , G , H , LC > ; fn or_filter (self , predicate : Predicate) -> Self :: Output { SelectStatement :: new (self . select , self . from , self . distinct , self . where_clause . or (predicate) , self . order , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
