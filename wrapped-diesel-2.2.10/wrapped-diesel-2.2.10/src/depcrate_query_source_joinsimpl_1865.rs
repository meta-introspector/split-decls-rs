// Generated macro for impl_1865 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1865 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1865"}
// Dependencies: {}
impl < Left , Right > QuerySource for Join < Left , Right , LeftOuter > where Left : QuerySource + AppendSelection < Nullable < Right :: DefaultSelection > > , Right : QuerySource , Left :: Output : AppearsOnTable < Self > , Self : Clone , { type FromClause = Self ; type DefaultSelection = self :: private :: SkipSelectableExpressionBoundCheckWrapper < Left :: Output > ; fn from_clause (& self) -> Self :: FromClause { self . clone () } fn default_selection (& self) -> Self :: DefaultSelection { self :: private :: SkipSelectableExpressionBoundCheckWrapper (self . left . source . append_selection (self . right . source . default_selection () . nullable ()) ,) } }
};
}
