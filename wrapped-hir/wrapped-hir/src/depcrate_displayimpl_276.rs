// Generated macro for impl_276 (impl)
macro_rules! Depcrate_displayimpl_276 {
() => {
// Module: crate::display
// Provides: {"impl_276"}
// Dependencies: {}
impl HirDisplay for Field { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { write_visibility (self . parent . module (f . db) . id , self . visibility (f . db) , f) ? ; write ! (f , "{}: " , self . name (f . db) . display (f . db , f . edition ())) ? ; self . ty (f . db) . hir_fmt (f) } }
};
}
