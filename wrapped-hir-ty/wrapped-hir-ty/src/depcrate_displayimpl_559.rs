// Generated macro for impl_559 (impl)
macro_rules! Depcrate_displayimpl_559 {
() => {
// Module: crate::display
// Provides: {"impl_559"}
// Dependencies: {}
impl < 'db , T : HirDisplayWithExpressionStore < 'db > > HirDisplay < 'db > for ExpressionStoreAdapter < '_ , T > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { T :: hir_fmt (& self . 0 , f , self . 1) } }
};
}
