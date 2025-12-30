// Generated macro for impl_283 (impl)
macro_rules! Depcrate_displayimpl_283 {
() => {
// Module: crate::display
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for GenericParam { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self { GenericParam :: TypeParam (it) => it . hir_fmt (f) , GenericParam :: ConstParam (it) => it . hir_fmt (f) , GenericParam :: LifetimeParam (it) => it . hir_fmt (f) , } } }
};
}
