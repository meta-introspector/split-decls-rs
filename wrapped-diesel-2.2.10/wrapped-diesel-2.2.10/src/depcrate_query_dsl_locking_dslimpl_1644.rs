// Generated macro for impl_1644 (impl)
macro_rules! Depcrate_query_dsl_locking_dslimpl_1644 {
() => {
// Module: crate::query_dsl::locking_dsl
// Provides: {"impl_1644"}
// Dependencies: {}
impl < T , Lock > LockingDsl < Lock > for T where T : Table + AsQuery < Query = SelectStatement < FromClause < T > > > , T :: DefaultSelection : Expression < SqlType = T :: SqlType > + ValidGrouping < () > , T :: SqlType : TypedExpressionType , { type Output = < SelectStatement < FromClause < T > > as LockingDsl < Lock > > :: Output ; fn with_lock (self , lock : Lock) -> Self :: Output { self . as_query () . with_lock (lock) } }
};
}
