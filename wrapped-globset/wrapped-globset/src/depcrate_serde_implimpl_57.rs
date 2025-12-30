// Generated macro for impl_57 (impl)
macro_rules! Depcrate_serde_implimpl_57 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_57"}
// Dependencies: {}
impl Serialize for Glob { fn serialize < S : Serializer > (& self , serializer : S ,) -> Result < S :: Ok , S :: Error > { serializer . serialize_str (self . glob ()) } }
};
}
