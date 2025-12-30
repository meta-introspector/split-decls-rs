// Generated macro for impl_1291 (impl)
macro_rules! Depcrate_query_builder_select_statementimpl_1291 {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"impl_1291"}
// Dependencies: {}
impl < From > QuerySource for SelectStatement < From > where From : AsQuerySource , < From :: QuerySource as QuerySource > :: DefaultSelection : SelectableExpression < Self > , { type FromClause = < From :: QuerySource as QuerySource > :: FromClause ; type DefaultSelection = < From :: QuerySource as QuerySource > :: DefaultSelection ; fn from_clause (& self) -> < From :: QuerySource as QuerySource > :: FromClause { self . from . as_query_source () . from_clause () } fn default_selection (& self) -> Self :: DefaultSelection { self . from . as_query_source () . default_selection () } }
};
}
