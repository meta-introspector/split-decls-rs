// Generated macro for impl_632 (impl)
macro_rules! Depcrate_displayimpl_632 {
() => {
// Module: crate::display
// Provides: {"impl_632"}
// Dependencies: {}
impl < T : ? Sized + HirDisplayWithExpressionStore > HirDisplayWithExpressionStore for & '_ T { fn hir_fmt (& self , f : & mut HirFormatter < '_ > , store : & ExpressionStore ,) -> Result < () , HirDisplayError > { T :: hir_fmt (& * * self , f , store) } }
};
}
