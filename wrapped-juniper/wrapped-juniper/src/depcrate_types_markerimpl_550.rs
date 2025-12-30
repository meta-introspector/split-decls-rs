// Generated macro for impl_550 (impl)
macro_rules! Depcrate_types_markerimpl_550 {
() => {
// Module: crate::types::marker
// Provides: {"impl_550"}
// Dependencies: {}
impl < S , T > GraphQLUnion < S > for & T where T : GraphQLUnion < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
