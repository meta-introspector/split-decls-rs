// Generated macro for impl_1833 (impl)
macro_rules! Depcrate_query_source_aliasing_joinsimpl_1833 {
() => {
// Module: crate::query_source::aliasing::joins
// Provides: {"impl_1833"}
// Dependencies: {}
impl < Left , Right , S , C > SelectableExpression < Join < Left , Right , LeftOuter > > for AliasedField < S , C > where Self : AppearsOnTable < Join < Left , Right , LeftOuter > > , Self : SelectableExpression < Left > , Left : QuerySource , Right : AppearsInFromClause < Alias < S > , Count = Never > + QuerySource , { }
};
}
