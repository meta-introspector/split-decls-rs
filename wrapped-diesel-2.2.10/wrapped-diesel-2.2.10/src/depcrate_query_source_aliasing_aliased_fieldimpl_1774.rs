// Generated macro for impl_1774 (impl)
macro_rules! Depcrate_query_source_aliasing_aliased_fieldimpl_1774 {
() => {
// Module: crate::query_source::aliasing::aliased_field
// Provides: {"impl_1774"}
// Dependencies: {}
impl < S , C > Expression for AliasedField < S , C > where S : AliasSource , C : Column < Table = S :: Target > + Expression , { type SqlType = C :: SqlType ; }
};
}
