// Generated macro for impl_296 (impl)
macro_rules! Depcrate_displayimpl_296 {
() => {
// Module: crate::display
// Provides: {"impl_296"}
// Dependencies: {}
impl HirDisplay for TypeAlias { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { write_visibility (self . module (f . db) . id , self . visibility (f . db) , f) ? ; let data = f . db . type_alias_signature (self . id) ; write ! (f , "type {}" , data . name . display (f . db , f . edition ())) ? ; let def_id = GenericDefId :: TypeAliasId (self . id) ; write_generic_params (def_id , f) ? ; if ! data . bounds . is_empty () { f . write_str (": ") ? ; f . write_joined (data . bounds . iter () . map (| bound | hir_display_with_store (bound , & data . store)) , " + " ,) ? ; } if let Some (ty) = data . ty { f . write_str (" = ") ? ; ty . hir_fmt (f , & data . store) ? ; } write_where_clause (def_id , f) ? ; Ok (()) } }
};
}
