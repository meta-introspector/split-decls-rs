// Generated macro for impl_71 (impl)
macro_rules! Depcrate_primitivesimpl_71 {
() => {
// Module: crate::primitives
// Provides: {"impl_71"}
// Dependencies: {}
impl < T , E > BakeSize for Result < T , E > where T : BakeSize , E : BakeSize , { fn borrows_size (& self) -> usize { self . as_ref () . map_or_else (BakeSize :: borrows_size , BakeSize :: borrows_size) } }
};
}
