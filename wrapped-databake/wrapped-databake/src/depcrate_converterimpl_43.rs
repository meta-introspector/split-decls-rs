// Generated macro for impl_43 (impl)
macro_rules! Depcrate_converterimpl_43 {
() => {
// Module: crate::converter
// Provides: {"impl_43"}
// Dependencies: {}
impl < B , T > Deref for IteratorAsRefSlice < B , T > where for < 'a > & 'a B : IntoIterator < Item = & 'a T > , T : Bake , { type Target = B ; # [inline] fn deref (& self) -> & Self :: Target { & self . 0 } }
};
}
