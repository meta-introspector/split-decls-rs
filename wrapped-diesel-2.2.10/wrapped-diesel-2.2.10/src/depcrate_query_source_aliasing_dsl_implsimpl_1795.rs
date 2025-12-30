// Generated macro for impl_1795 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1795 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1795"}
// Dependencies: {}
impl < 'a , S , DB > BoxedDsl < 'a , DB > for Alias < S > where Alias < S > : QuerySource + AsQuery < Query = SelectStatement < FromClause < Alias < S > > > > , SelectStatement < FromClause < Alias < S > > > : BoxedDsl < 'a , DB > , < Alias < S > as QuerySource > :: DefaultSelection : Expression < SqlType = < Alias < S > as AsQuery > :: SqlType > + ValidGrouping < () > , < Alias < S > as AsQuery > :: SqlType : TypedExpressionType , { type Output = dsl :: IntoBoxed < 'a , SelectStatement < FromClause < Alias < S > > > , DB > ; fn internal_into_boxed (self) -> Self :: Output { self . as_query () . internal_into_boxed () } }
};
}
