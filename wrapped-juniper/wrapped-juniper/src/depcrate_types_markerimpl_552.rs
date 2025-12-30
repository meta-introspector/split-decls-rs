// Generated macro for impl_552 (impl)
macro_rules! Depcrate_types_markerimpl_552 {
() => {
// Module: crate::types::marker
// Provides: {"impl_552"}
// Dependencies: {}
impl < S , T > GraphQLUnion < S > for Arc < T > where T : GraphQLUnion < S > + ? Sized , S : ScalarValue , { # [inline] fn mark () { T :: mark () } }
};
}
