// Generated macro for impl_278 (impl)
macro_rules! Depcrate_displayimpl_278 {
() => {
// Module: crate::display
// Provides: {"impl_278"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for TupleField { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write ! (f , "pub {}: " , self . name () . display (f . db , f . edition ())) ? ; self . ty (f . db) . hir_fmt (f) } }
};
}
