// Generated macro for impl_1748 (impl)
macro_rules! Depcrate_query_source_aliasing_aliasimpl_1748 {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"impl_1748"}
// Dependencies: {}
impl < S > AsQuery for Alias < S > where S : AliasSource , S :: Target : AsQuery , Self : QuerySource , < Self as QuerySource > :: DefaultSelection : ValidGrouping < () > , { type SqlType = < < Self as QuerySource > :: DefaultSelection as Expression > :: SqlType ; type Query = SelectStatement < FromClause < Self > > ; fn as_query (self) -> Self :: Query { SelectStatement :: simple (self) } }
};
}
