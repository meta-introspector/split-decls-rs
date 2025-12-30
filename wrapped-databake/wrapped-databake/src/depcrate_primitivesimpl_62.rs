// Generated macro for impl_62 (impl)
macro_rules! Depcrate_primitivesimpl_62 {
() => {
// Module: crate::primitives
// Provides: {"impl_62"}
// Dependencies: {}
impl < T > BakeSize for & [T] where T : BakeSize , { fn borrows_size (& self) -> usize { std :: mem :: size_of_val (* self) + self . iter () . map (BakeSize :: borrows_size) . sum :: < usize > () } }
};
}
