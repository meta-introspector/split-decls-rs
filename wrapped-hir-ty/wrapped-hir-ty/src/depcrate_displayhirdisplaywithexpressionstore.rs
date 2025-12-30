// Generated macro for HirDisplayWithExpressionStore (trait)
macro_rules! Depcrate_displayHirDisplayWithExpressionStore {
() => {
// Module: crate::display
// Provides: {"HirDisplayWithExpressionStore"}
// Dependencies: {}
pub trait HirDisplayWithExpressionStore < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db > , store : & ExpressionStore ,) -> Result < () , HirDisplayError > ; }
};
}
