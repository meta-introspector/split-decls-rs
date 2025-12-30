// Generated macro for impl_1477 (impl)
macro_rules! Depcrate_query_builder_where_clauseimpl_1477 {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"impl_1477"}
// Dependencies: {}
impl < Expr , Predicate > WhereAnd < Predicate > for WhereClause < Expr > where Expr : Expression , Expr :: SqlType : BoolOrNullableBool , Predicate : Expression , Predicate :: SqlType : BoolOrNullableBool , { type Output = WhereClause < Grouped < And < Expr , Predicate > > > ; fn and (self , predicate : Predicate) -> Self :: Output { WhereClause (Grouped (And :: new (self . 0 , predicate))) } }
};
}
