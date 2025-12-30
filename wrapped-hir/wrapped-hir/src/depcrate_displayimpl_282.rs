// Generated macro for impl_282 (impl)
macro_rules! Depcrate_displayimpl_282 {
() => {
// Module: crate::display
// Provides: {"impl_282"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for ExternCrateDecl { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write_visibility (self . module (f . db) . id , self . visibility (f . db) , f) ? ; f . write_str ("extern crate ") ? ; write ! (f , "{}" , self . name (f . db) . display (f . db , f . edition ())) ? ; if let Some (alias) = self . alias (f . db) { write ! (f , " as {}" , alias . display (f . edition ())) ? ; } Ok (()) } }
};
}
