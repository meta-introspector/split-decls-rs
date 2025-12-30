// Generated macro for impl_542 (impl)
macro_rules! Depcrate_types_markerimpl_542 {
() => {
// Module: crate::types::marker
// Provides: {"impl_542"}
// Dependencies: {}
impl < S , T > GraphQLObject < S > for & T where T : GraphQLObject < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
