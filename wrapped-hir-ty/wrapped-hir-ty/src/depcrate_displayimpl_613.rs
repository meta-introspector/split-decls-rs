// Generated macro for impl_613 (impl)
macro_rules! Depcrate_displayimpl_613 {
() => {
// Module: crate::display
// Provides: {"impl_613"}
// Dependencies: {}
impl HirDisplay for BoundVar { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { write ! (f , "?{}.{}" , self . debruijn . depth () , self . index) } }
};
}
