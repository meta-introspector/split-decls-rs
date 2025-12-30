// Generated macro for impl_544 (impl)
macro_rules! Depcrate_types_markerimpl_544 {
() => {
// Module: crate::types::marker
// Provides: {"impl_544"}
// Dependencies: {}
impl < S , T > GraphQLObject < S > for Arc < T > where T : GraphQLObject < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
