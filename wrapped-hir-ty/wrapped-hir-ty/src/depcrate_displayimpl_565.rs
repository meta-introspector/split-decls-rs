// Generated macro for impl_565 (impl)
macro_rules! Depcrate_displayimpl_565 {
() => {
// Module: crate::display
// Provides: {"impl_565"}
// Dependencies: {}
impl < 'db > HirDisplayWithExpressionStore < 'db > for hir_def :: expr_store :: path :: GenericArg { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db > , store : & ExpressionStore ,) -> Result < () , HirDisplayError > { match self { hir_def :: expr_store :: path :: GenericArg :: Type (ty) => ty . hir_fmt (f , store) , hir_def :: expr_store :: path :: GenericArg :: Const (_c) => { write ! (f , "<expr>") } hir_def :: expr_store :: path :: GenericArg :: Lifetime (lifetime) => lifetime . hir_fmt (f , store) , } } }
};
}
