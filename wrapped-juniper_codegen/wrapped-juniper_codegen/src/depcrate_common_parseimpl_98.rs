// Generated macro for impl_98 (impl)
macro_rules! Depcrate_common_parseimpl_98 {
() => {
// Module: crate::common::parse
// Provides: {"impl_98"}
// Dependencies: {}
impl GenericsExt for syn :: Generics { fn move_bounds_to_where_clause (& mut self) { use syn :: GenericParam as P ; let _ = self . make_where_clause () ; let where_clause = self . where_clause . as_mut () . unwrap () ; for p in & mut self . params { match p { P :: Type (p) => { if p . colon_token . is_some () { p . colon_token = None ; let bounds = mem :: take (& mut p . bounds) ; let ty = & p . ident ; where_clause . predicates . push (parse_quote ! { # ty : # bounds }) ; } } P :: Lifetime (p) => { if p . colon_token . is_some () { p . colon_token = None ; let bounds = mem :: take (& mut p . bounds) ; let lt = & p . lifetime ; where_clause . predicates . push (parse_quote ! { # lt : # bounds }) ; } } P :: Const (_) => { } } } } fn replace_type_with_defaults (& self , ty : & mut syn :: Type) { ReplaceWithDefaults (self) . visit_type_mut (ty) } fn replace_type_path_with_defaults (& self , ty : & mut syn :: TypePath) { ReplaceWithDefaults (self) . visit_type_path_mut (ty) } }
};
}
