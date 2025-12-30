// Generated macro for impl_299 (impl)
macro_rules! Depcrate_displayimpl_299 {
() => {
// Module: crate::display
// Provides: {"impl_299"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for Crate { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self . display_name (f . db) { Some (name) => write ! (f , "extern crate {name}") , None => f . write_str ("extern crate {unknown}") , } } }
};
}
