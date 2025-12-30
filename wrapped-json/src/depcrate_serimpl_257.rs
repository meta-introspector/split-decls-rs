// Generated macro for impl_257 (impl)
macro_rules! Depcrate_serimpl_257 {
() => {
// Module: crate::ser
// Provides: {"impl_257"}
// Dependencies: {}
impl < 'a , W > Serializer < W , PrettyFormatter < 'a > > where W : io :: Write , { # [doc = " Creates a new JSON pretty print serializer."] # [inline] pub fn pretty (writer : W) -> Self { Serializer :: with_formatter (writer , PrettyFormatter :: new ()) } }
};
}
