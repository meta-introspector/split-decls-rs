// Generated macro for impl_1750 (impl)
macro_rules! Depcrate_query_source_aliasing_aliasimpl_1750 {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"impl_1750"}
// Dependencies: {}
impl < S , QS > AppearsInFromClause < QS > for Alias < S > where S : AliasSource , S :: Target : AliasAppearsInFromClause < S , QS > , { type Count = < S :: Target as AliasAppearsInFromClause < S , QS > > :: Count ; }
};
}
