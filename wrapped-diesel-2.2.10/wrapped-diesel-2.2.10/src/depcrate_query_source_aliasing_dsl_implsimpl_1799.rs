// Generated macro for impl_1799 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1799 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1799"}
// Dependencies: {}
impl < S , Expr > GroupByDsl < Expr > for Alias < S > where Expr : Expression , Self : QuerySource + AsQuery < Query = SelectStatement < FromClause < Self > > > , < Self as QuerySource > :: DefaultSelection : Expression < SqlType = < Self as AsQuery > :: SqlType > + ValidGrouping < () > , < Self as AsQuery > :: SqlType : TypedExpressionType , < Self as AsQuery > :: Query : GroupByDsl < Expr > , { type Output = dsl :: GroupBy < SelectStatement < FromClause < Self > > , Expr > ; fn group_by (self , expr : Expr) -> dsl :: GroupBy < Self , Expr > { GroupByDsl :: group_by (self . as_query () , expr) } }
};
}
