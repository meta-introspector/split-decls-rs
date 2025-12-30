// Generated macro for process_field (function)
macro_rules! Depcrate_internals_serialize_structsprocess_field {
() => {
// Module: crate::internals::serialize::structs
// Provides: {"process_field"}
// Dependencies: {}
fn process_field (field : & syn :: Field , field_id : serialize :: FieldId , cratename : & Path , generics : & mut serialize :: GenericsOutput , body : & mut TokenStream2 ,) -> syn :: Result < () > { let parsed = field :: Attributes :: parse (& field . attrs) ? ; let needs_bounds_derive = parsed . needs_bounds_derive (BoundType :: Serialize) ; generics . overrides . extend (parsed . collect_bounds (BoundType :: Serialize)) ; if ! parsed . skip { let delta = field_id . serialize_output (cratename , parsed . serialize_with) ; body . extend (delta) ; if needs_bounds_derive { generics . serialize_visitor . visit_field (field) ; } } Ok (()) }
};
}
