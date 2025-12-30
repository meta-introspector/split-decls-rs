// Generated macro for impl_272 (impl)
macro_rules! Depcrate_displayimpl_272 {
() => {
// Module: crate::display
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for Struct { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { let module_id = self . module (f . db) . id ; write_visibility (module_id , self . visibility (f . db) , f) ? ; f . write_str ("struct ") ? ; write ! (f , "{}" , self . name (f . db) . display (f . db , f . edition ())) ? ; let def_id = GenericDefId :: AdtId (AdtId :: StructId (self . id)) ; write_generic_params (def_id , f) ? ; let variant_data = self . variant_fields (f . db) ; match self . kind (f . db) { StructKind :: Tuple => { f . write_char ('(') ? ; let mut it = variant_data . fields () . iter () . peekable () ; while let Some ((id , _)) = it . next () { let field = Field { parent : (* self) . into () , id } ; write_visibility (module_id , field . visibility (f . db) , f) ? ; field . ty (f . db) . hir_fmt (f) ? ; if it . peek () . is_some () { f . write_str (", ") ? ; } } f . write_char (')') ? ; write_where_clause (def_id , f) ? ; } StructKind :: Record => { let has_where_clause = write_where_clause (def_id , f) ? ; if let Some (limit) = f . entity_limit { write_fields (& self . fields (f . db) , has_where_clause , limit , false , f) ? ; } } StructKind :: Unit => _ = write_where_clause (def_id , f) ? , } Ok (()) } }
};
}
