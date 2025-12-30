// Generated macro for impl_3515 (impl)
macro_rules! Depcrate_pg_query_builder_onlyimpl_3515 {
() => {
// Module: crate::pg::query_builder::only
// Provides: {"impl_3515"}
// Dependencies: {}
impl < S > QuerySource for Only < S > where S : Table + Clone , < S as QuerySource > :: DefaultSelection : ValidGrouping < () > + SelectableExpression < Only < S > > , { type FromClause = Self ; type DefaultSelection = < S as QuerySource > :: DefaultSelection ; fn from_clause (& self) -> Self :: FromClause { self . clone () } fn default_selection (& self) -> Self :: DefaultSelection { self . source . default_selection () } }
};
}
