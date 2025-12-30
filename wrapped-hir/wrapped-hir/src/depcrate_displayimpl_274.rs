// Generated macro for impl_274 (impl)
macro_rules! Depcrate_displayimpl_274 {
() => {
// Module: crate::display
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for Union { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write_visibility (self . module (f . db) . id , self . visibility (f . db) , f) ? ; f . write_str ("union ") ? ; write ! (f , "{}" , self . name (f . db) . display (f . db , f . edition ())) ? ; let def_id = GenericDefId :: AdtId (AdtId :: UnionId (self . id)) ; write_generic_params (def_id , f) ? ; let has_where_clause = write_where_clause (def_id , f) ? ; if let Some (limit) = f . entity_limit { write_fields (& self . fields (f . db) , has_where_clause , limit , false , f) ? ; } Ok (()) } }
};
}
