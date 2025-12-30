// Generated macro for impl_1558 (impl)
macro_rules! Depcrate_query_dsl_boxed_dslimpl_1558 {
() => {
// Module: crate::query_dsl::boxed_dsl
// Provides: {"impl_1558"}
// Dependencies: {}
impl < 'a , T , DB > BoxedDsl < 'a , DB > for T where T : Table + AsQuery < Query = SelectStatement < FromClause < T > > > , SelectStatement < FromClause < T > > : BoxedDsl < 'a , DB > , T :: DefaultSelection : Expression < SqlType = T :: SqlType > + ValidGrouping < () > , T :: SqlType : TypedExpressionType , { type Output = dsl :: IntoBoxed < 'a , SelectStatement < FromClause < T > > , DB > ; fn internal_into_boxed (self) -> Self :: Output { self . as_query () . internal_into_boxed () } }
};
}
