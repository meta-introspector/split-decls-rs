// Generated macro for impl_44 (impl)
macro_rules! Depcrate_converterimpl_44 {
() => {
// Module: crate::converter
// Provides: {"impl_44"}
// Dependencies: {}
impl < B , T > DerefMut for IteratorAsRefSlice < B , T > where for < 'a > & 'a B : IntoIterator < Item = & 'a T > , T : Bake , { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
};
}
