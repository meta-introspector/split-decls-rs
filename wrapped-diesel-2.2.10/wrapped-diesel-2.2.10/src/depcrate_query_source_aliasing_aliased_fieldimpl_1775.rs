// Generated macro for impl_1775 (impl)
macro_rules! Depcrate_query_source_aliasing_aliased_fieldimpl_1775 {
() => {
// Module: crate::query_source::aliasing::aliased_field
// Provides: {"impl_1775"}
// Dependencies: {}
impl < S , C > SelectableExpression < Alias < S > > for AliasedField < S , C > where S : AliasSource , C : Column < Table = S :: Target > , Self : AppearsOnTable < Alias < S > > , { }
};
}
