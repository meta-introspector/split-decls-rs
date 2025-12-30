// Generated macro for impl_548 (impl)
macro_rules! Depcrate_types_markerimpl_548 {
() => {
// Module: crate::types::marker
// Provides: {"impl_548"}
// Dependencies: {}
impl < S , T > GraphQLInterface < S > for Arc < T > where T : GraphQLInterface < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
