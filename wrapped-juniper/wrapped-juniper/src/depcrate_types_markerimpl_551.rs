// Generated macro for impl_551 (impl)
macro_rules! Depcrate_types_markerimpl_551 {
() => {
// Module: crate::types::marker
// Provides: {"impl_551"}
// Dependencies: {}
impl < S , T > GraphQLUnion < S > for Box < T > where T : GraphQLUnion < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
