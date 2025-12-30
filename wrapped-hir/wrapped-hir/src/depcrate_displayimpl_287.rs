// Generated macro for impl_287 (impl)
macro_rules! Depcrate_displayimpl_287 {
() => {
// Module: crate::display
// Provides: {"impl_287"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for ConstParam { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write ! (f , "const {}: " , self . name (f . db) . display (f . db , f . edition ())) ? ; self . ty (f . db) . hir_fmt (f) } }
};
}
