// Generated macro for impl_1868 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1868 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1868"}
// Dependencies: {}
impl < Join , On > QuerySource for JoinOn < Join , On > where Join : QuerySource , On : AppearsOnTable < Join :: FromClause > + Clone , On :: SqlType : BoolOrNullableBool , Join :: DefaultSelection : SelectableExpression < Self > , { type FromClause = Grouped < nodes :: InfixNode < Join :: FromClause , On , OnKeyword > > ; type DefaultSelection = Join :: DefaultSelection ; fn from_clause (& self) -> Self :: FromClause { Grouped (nodes :: InfixNode :: new (self . join . from_clause () , self . on . clone () , OnKeyword ,)) } fn default_selection (& self) -> Self :: DefaultSelection { self . join . default_selection () } }
};
}
