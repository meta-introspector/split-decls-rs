// Generated macro for impl_562 (impl)
macro_rules! Depcrate_displayimpl_562 {
() => {
// Module: crate::display
// Provides: {"impl_562"}
// Dependencies: {}
impl < 'db > HirDisplayWithExpressionStore < 'db > for ConstRef { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db > , _store : & ExpressionStore ,) -> Result < () , HirDisplayError > { write ! (f , "{{const}}") ? ; Ok (()) } }
};
}
