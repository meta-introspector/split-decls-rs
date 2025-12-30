// Generated macro for impl_1834 (impl)
macro_rules! Depcrate_query_source_aliasing_joinsimpl_1834 {
() => {
// Module: crate::query_source::aliasing::joins
// Provides: {"impl_1834"}
// Dependencies: {}
impl < Left , Right , S , C > SelectableExpression < Join < Left , Right , Inner > > for AliasedField < S , C > where Self : AppearsOnTable < Join < Left , Right , Inner > > , Left : AppearsInFromClause < Alias < S > > + QuerySource , Right : AppearsInFromClause < Alias < S > > + QuerySource , (Left :: Count , Right :: Count) : Pick < Left , Right > , Self : SelectableExpression < < (Left :: Count , Right :: Count) as Pick < Left , Right > > :: Selection > , { }
};
}
