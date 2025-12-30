// Generated macro for impl_65 (impl)
macro_rules! Depcrate_primitivesimpl_65 {
() => {
// Module: crate::primitives
// Provides: {"impl_65"}
// Dependencies: {}
impl < T , const N : usize > BakeSize for [T ; N] where T : BakeSize , { fn borrows_size (& self) -> usize { self . iter () . map (BakeSize :: borrows_size) . sum () } }
};
}
