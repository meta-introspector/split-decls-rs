// Generated macro for impl_546 (impl)
macro_rules! Depcrate_types_markerimpl_546 {
() => {
// Module: crate::types::marker
// Provides: {"impl_546"}
// Dependencies: {}
impl < S , T > GraphQLInterface < S > for & T where T : GraphQLInterface < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
