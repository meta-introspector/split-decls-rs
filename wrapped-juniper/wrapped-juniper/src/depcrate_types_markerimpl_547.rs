// Generated macro for impl_547 (impl)
macro_rules! Depcrate_types_markerimpl_547 {
() => {
// Module: crate::types::marker
// Provides: {"impl_547"}
// Dependencies: {}
impl < S , T > GraphQLInterface < S > for Box < T > where T : GraphQLInterface < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
