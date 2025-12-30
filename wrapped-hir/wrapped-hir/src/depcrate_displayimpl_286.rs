// Generated macro for impl_286 (impl)
macro_rules! Depcrate_displayimpl_286 {
() => {
// Module: crate::display
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for LifetimeParam { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write ! (f , "{}" , self . name (f . db) . display (f . db , f . edition ())) } }
};
}
