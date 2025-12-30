// Generated macro for impl_293 (impl)
macro_rules! Depcrate_displayimpl_293 {
() => {
// Module: crate::display
// Provides: {"impl_293"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for Static { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write_visibility (self . module (f . db) . id , self . visibility (f . db) , f) ? ; let data = f . db . static_signature (self . id) ; f . write_str ("static ") ? ; if data . flags . contains (StaticFlags :: MUTABLE) { f . write_str ("mut ") ? ; } write ! (f , "{}: " , data . name . display (f . db , f . edition ())) ? ; data . type_ref . hir_fmt (f , & data . store) ? ; Ok (()) } }
};
}
