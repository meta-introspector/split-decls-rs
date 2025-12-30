// Generated macro for impl_46 (impl)
macro_rules! Depcrate_converterimpl_46 {
() => {
// Module: crate::converter
// Provides: {"impl_46"}
// Dependencies: {}
impl < B , T > FromIterator < T > for IteratorAsRefSlice < B , T > where B : FromIterator < T > , for < 'a > & 'a B : IntoIterator < Item = & 'a T > , T : Bake , { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { Self (iter . into_iter () . collect () , PhantomData) } }
};
}
