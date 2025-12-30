// Generated macro for impl_543 (impl)
macro_rules! Depcrate_types_markerimpl_543 {
() => {
// Module: crate::types::marker
// Provides: {"impl_543"}
// Dependencies: {}
impl < S , T > GraphQLObject < S > for Box < T > where T : GraphQLObject < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
