// Generated macro for impl_1478 (impl)
macro_rules! Depcrate_query_builder_where_clauseimpl_1478 {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"impl_1478"}
// Dependencies: {}
impl < Expr , Predicate > WhereOr < Predicate > for WhereClause < Expr > where Expr : Expression , Expr :: SqlType : BoolOrNullableBool , Predicate : Expression , Predicate :: SqlType : BoolOrNullableBool , { type Output = WhereClause < Grouped < Or < Expr , Predicate > > > ; fn or (self , predicate : Predicate) -> Self :: Output { WhereClause (Grouped (Or :: new (self . 0 , predicate))) } }
};
}
