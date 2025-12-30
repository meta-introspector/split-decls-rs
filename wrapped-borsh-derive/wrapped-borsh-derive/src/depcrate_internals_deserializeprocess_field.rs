// Generated macro for process_field (function)
macro_rules! Depcrate_internals_deserializeprocess_field {
() => {
// Module: crate::internals::deserialize
// Provides: {"process_field"}
// Dependencies: {}
fn process_field (field : & syn :: Field , cratename : & Path , body : & mut TokenStream2 , generics : & mut GenericsOutput ,) -> syn :: Result < () > { let parsed = field :: Attributes :: parse (& field . attrs) ? ; generics . overrides . extend (parsed . collect_bounds (BoundType :: Deserialize)) ; let needs_bounds_derive = parsed . needs_bounds_derive (BoundType :: Deserialize) ; let field_name = field . ident . as_ref () ; let delta = if parsed . skip { if needs_bounds_derive { generics . default_visitor . visit_field (field) ; } field_default_output (field_name) } else { if needs_bounds_derive { generics . deserialize_visitor . visit_field (field) ; } field_output (field_name , cratename , parsed . deserialize_with) } ; body . extend (delta) ; Ok (()) }
};
}
