// Generated macro for impl_284 (impl)
macro_rules! Depcrate_displayimpl_284 {
() => {
// Module: crate::display
// Provides: {"impl_284"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for TypeOrConstParam { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self . split (f . db) { either :: Either :: Left (it) => it . hir_fmt (f) , either :: Either :: Right (it) => it . hir_fmt (f) , } } }
};
}
