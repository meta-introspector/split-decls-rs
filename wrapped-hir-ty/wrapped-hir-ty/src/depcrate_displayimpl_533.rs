// Generated macro for impl_533 (impl)
macro_rules! Depcrate_displayimpl_533 {
() => {
// Module: crate::display
// Provides: {"impl_533"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for GenericArg < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self { GenericArg :: Ty (ty) => ty . hir_fmt (f) , GenericArg :: Lifetime (lt) => lt . hir_fmt (f) , GenericArg :: Const (c) => c . hir_fmt (f) , } } }
};
}
