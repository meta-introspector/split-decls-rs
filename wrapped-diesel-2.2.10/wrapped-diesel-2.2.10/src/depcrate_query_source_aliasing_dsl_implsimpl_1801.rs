// Generated macro for impl_1801 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1801 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1801"}
// Dependencies: {}
impl < S , Lock > LockingDsl < Lock > for Alias < S > where Self : QuerySource + AsQuery < Query = SelectStatement < FromClause < Self > > > , < Self as QuerySource > :: DefaultSelection : Expression < SqlType = < Self as AsQuery > :: SqlType > + ValidGrouping < () > , < Self as AsQuery > :: SqlType : TypedExpressionType , { type Output = < SelectStatement < FromClause < Self > > as LockingDsl < Lock > > :: Output ; fn with_lock (self , lock : Lock) -> Self :: Output { self . as_query () . with_lock (lock) } }
};
}
