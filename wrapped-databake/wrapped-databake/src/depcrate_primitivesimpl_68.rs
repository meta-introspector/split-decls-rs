// Generated macro for impl_68 (impl)
macro_rules! Depcrate_primitivesimpl_68 {
() => {
// Module: crate::primitives
// Provides: {"impl_68"}
// Dependencies: {}
impl < T > BakeSize for Option < T > where T : BakeSize , { fn borrows_size (& self) -> usize { self . as_ref () . map (BakeSize :: borrows_size) . unwrap_or_default () } }
};
}
