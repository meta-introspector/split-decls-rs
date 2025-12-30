// Generated macro for impl_42 (impl)
macro_rules! Depcrate_converterimpl_42 {
() => {
// Module: crate::converter
// Provides: {"impl_42"}
// Dependencies: {}
impl < B , T > BakeSize for IteratorAsRefSlice < B , T > where for < 'a > & 'a B : IntoIterator < Item = & 'a T > , T : BakeSize , { fn borrows_size (& self) -> usize { self . 0 . into_iter () . map (| x | x . borrows_size ()) . sum () } }
};
}
