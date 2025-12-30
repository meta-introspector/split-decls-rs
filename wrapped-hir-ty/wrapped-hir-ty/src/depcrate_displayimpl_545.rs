// Generated macro for impl_545 (impl)
macro_rules! Depcrate_displayimpl_545 {
() => {
// Module: crate::display
// Provides: {"impl_545"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for Term < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self { Term :: Ty (it) => it . hir_fmt (f) , Term :: Const (it) => it . hir_fmt (f) , } } }
};
}
