// Generated macro for impl_298 (impl)
macro_rules! Depcrate_displayimpl_298 {
() => {
// Module: crate::display
// Provides: {"impl_298"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for Module { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self . parent (f . db) { Some (m) => write_visibility (m . id , self . visibility (f . db) , f) ? , None => { return match self . krate (f . db) . display_name (f . db) { Some (name) => write ! (f , "extern crate {name}") , None => f . write_str ("extern crate {unknown}") , } ; } } match self . name (f . db) { Some (name) => write ! (f , "mod {}" , name . display (f . db , f . edition ())) , None => f . write_str ("mod {unknown}") , } } }
};
}
