// Generated macro for impl_3517 (impl)
macro_rules! Depcrate_pg_query_builder_onlyimpl_3517 {
() => {
// Module: crate::pg::query_builder::only
// Provides: {"impl_3517"}
// Dependencies: {}
impl < S > AsQuery for Only < S > where S : Table + Clone , < S as QuerySource > :: DefaultSelection : ValidGrouping < () > + SelectableExpression < Only < S > > , { type SqlType = < < Self as QuerySource > :: DefaultSelection as Expression > :: SqlType ; type Query = SelectStatement < FromClause < Self > > ; fn as_query (self) -> Self :: Query { SelectStatement :: simple (self) } }
};
}
