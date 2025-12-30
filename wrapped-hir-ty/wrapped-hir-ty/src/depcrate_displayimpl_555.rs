// Generated macro for impl_555 (impl)
macro_rules! Depcrate_displayimpl_555 {
() => {
// Module: crate::display
// Provides: {"impl_555"}
// Dependencies: {}
impl < 'db , T : ? Sized + HirDisplayWithExpressionStore < 'db > > HirDisplayWithExpressionStore < 'db > for & '_ T { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db > , store : & ExpressionStore ,) -> Result < () , HirDisplayError > { T :: hir_fmt (& * * self , f , store) } }
};
}
