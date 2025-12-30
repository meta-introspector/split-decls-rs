// Generated macro for impl_45 (impl)
macro_rules! Depcrate_converterimpl_45 {
() => {
// Module: crate::converter
// Provides: {"impl_45"}
// Dependencies: {}
impl < B , T > From < B > for IteratorAsRefSlice < B , T > where for < 'a > & 'a B : IntoIterator < Item = & 'a T > , T : Bake , { # [inline] fn from (value : B) -> Self { Self (value , PhantomData) } }
};
}
