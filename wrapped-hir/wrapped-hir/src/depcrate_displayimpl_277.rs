// Generated macro for impl_277 (impl)
macro_rules! Depcrate_displayimpl_277 {
() => {
// Module: crate::display
// Provides: {"impl_277"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for Field { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write_visibility (self . parent . module (f . db) . id , self . visibility (f . db) , f) ? ; write ! (f , "{}: " , self . name (f . db) . display (f . db , f . edition ())) ? ; self . ty (f . db) . hir_fmt (f) } }
};
}
