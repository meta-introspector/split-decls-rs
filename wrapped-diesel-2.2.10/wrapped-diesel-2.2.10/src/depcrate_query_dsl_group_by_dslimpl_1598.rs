// Generated macro for impl_1598 (impl)
macro_rules! Depcrate_query_dsl_group_by_dslimpl_1598 {
() => {
// Module: crate::query_dsl::group_by_dsl
// Provides: {"impl_1598"}
// Dependencies: {}
impl < T , Expr > GroupByDsl < Expr > for T where Expr : Expression , T : Table + AsQuery < Query = SelectStatement < FromClause < T > > > , T :: DefaultSelection : Expression < SqlType = T :: SqlType > + ValidGrouping < () > , T :: SqlType : TypedExpressionType , T :: Query : GroupByDsl < Expr > , { type Output = dsl :: GroupBy < SelectStatement < FromClause < T > > , Expr > ; fn group_by (self , expr : Expr) -> dsl :: GroupBy < Self , Expr > { self . as_query () . group_by (expr) } }
};
}
