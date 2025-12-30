// Generated macro for process_field (function)
macro_rules! Depcrate_internals_serialize_enumsprocess_field {
() => {
// Module: crate::internals::serialize::enums
// Provides: {"process_field"}
// Dependencies: {}
fn process_field (field : & syn :: Field , field_id : serialize :: FieldId , cratename : & Path , generics : & mut serialize :: GenericsOutput , output : & mut VariantFields ,) -> syn :: Result < () > { let parsed = field :: Attributes :: parse (& field . attrs) ? ; let needs_bounds_derive = parsed . needs_bounds_derive (BoundType :: Serialize) ; generics . overrides . extend (parsed . collect_bounds (BoundType :: Serialize)) ; let field_variant_header = field_id . enum_variant_header (parsed . skip) ; if let Some (field_variant_header) = field_variant_header { output . header . extend (field_variant_header) ; } if ! parsed . skip { let delta = field_id . serialize_output (cratename , parsed . serialize_with) ; output . body . extend (delta) ; if needs_bounds_derive { generics . serialize_visitor . visit_field (field) ; } } Ok (()) }
};
}
