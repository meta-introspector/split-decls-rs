// Generated macro for impl_292 (impl)
macro_rules! Depcrate_displayimpl_292 {
() => {
// Module: crate::display
// Provides: {"impl_292"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for Const { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { let db = f . db ; let container = self . as_assoc_item (db) . map (| it | it . container (db)) ; let mut module = self . module (db) ; if let Some (AssocItemContainer :: Impl (_)) = container { module = module . nearest_non_block_module (db) ; } write_visibility (module . id , self . visibility (db) , f) ? ; let data = db . const_signature (self . id) ; f . write_str ("const ") ? ; match & data . name { Some (name) => write ! (f , "{}: " , name . display (f . db , f . edition ())) ? , None => f . write_str ("_: ") ? , } data . type_ref . hir_fmt (f , & data . store) ? ; Ok (()) } }
};
}
