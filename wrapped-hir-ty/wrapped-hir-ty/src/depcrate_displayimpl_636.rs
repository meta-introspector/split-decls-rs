// Generated macro for impl_636 (impl)
macro_rules! Depcrate_displayimpl_636 {
() => {
// Module: crate::display
// Provides: {"impl_636"}
// Dependencies: {}
impl < T : HirDisplayWithExpressionStore > HirDisplay for ExpressionStoreAdapter < '_ , T > { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { T :: hir_fmt (& self . 0 , f , self . 1) } }
};
}
