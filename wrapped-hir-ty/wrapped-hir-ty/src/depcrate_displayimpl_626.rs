// Generated macro for impl_626 (impl)
macro_rules! Depcrate_displayimpl_626 {
() => {
// Module: crate::display
// Provides: {"impl_626"}
// Dependencies: {}
impl HirDisplay for LifetimeOutlives { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { self . a . hir_fmt (f) ? ; write ! (f , ": ") ? ; self . b . hir_fmt (f) } }
};
}
