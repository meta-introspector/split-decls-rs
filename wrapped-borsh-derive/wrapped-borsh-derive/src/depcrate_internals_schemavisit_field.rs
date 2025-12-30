// Generated macro for visit_field (function)
macro_rules! Depcrate_internals_schemavisit_field {
() => {
// Module: crate::internals::schema
// Provides: {"visit_field"}
// Dependencies: {}
fn visit_field (field : & Field , visitor : & mut generics :: FindTyParams) -> syn :: Result < () > { let parsed = field :: Attributes :: parse (& field . attrs) ? ; let needs_schema_params_derive = parsed . needs_schema_params_derive () ; let schema_attrs = parsed . schema ; if ! parsed . skip { if needs_schema_params_derive { visitor . visit_field (field) ; } if let Some (schema_attrs) = schema_attrs { if let Some (schema_params) = schema_attrs . params { for field :: schema :: ParameterOverride { order_param , override_type , .. } in schema_params { visitor . param_associated_type_insert (order_param , override_type) ; } } } } Ok (()) }
};
}
