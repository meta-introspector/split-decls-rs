// Generated macro for impl_1745 (impl)
macro_rules! Depcrate_query_source_aliasing_aliasimpl_1745 {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"impl_1745"}
// Dependencies: {}
impl < S > QuerySource for Alias < S > where Self : Clone , S : AliasSource , S :: Target : QuerySource , < S :: Target as QuerySource > :: DefaultSelection : FieldAliasMapper < S > , < < S :: Target as QuerySource > :: DefaultSelection as FieldAliasMapper < S > > :: Out : SelectableExpression < Self > , { type FromClause = Self ; type DefaultSelection = < < S :: Target as QuerySource > :: DefaultSelection as FieldAliasMapper < S > > :: Out ; fn from_clause (& self) -> Self :: FromClause { self . clone () } fn default_selection (& self) -> Self :: DefaultSelection { self . fields (self . source . target () . default_selection ()) } }
};
}
