// Generated macro for impl_1772 (impl)
macro_rules! Depcrate_query_source_aliasing_aliased_fieldimpl_1772 {
() => {
// Module: crate::query_source::aliasing::aliased_field
// Provides: {"impl_1772"}
// Dependencies: {}
impl < QS , S , C > AppearsOnTable < QS > for AliasedField < S , C > where S : AliasSource , QS : AppearsInFromClause < Alias < S > , Count = Once > , C : Column < Table = S :: Target > , { }
};
}
