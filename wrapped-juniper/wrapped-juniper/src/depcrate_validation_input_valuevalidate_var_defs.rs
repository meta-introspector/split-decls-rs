// Generated macro for validate_var_defs (function)
macro_rules! Depcrate_validation_input_valuevalidate_var_defs {
() => {
// Module: crate::validation::input_value
// Provides: {"validate_var_defs"}
// Dependencies: {}
fn validate_var_defs < S > (values : & Variables < S > , var_defs : & VariablesDefinition < S > , schema : & SchemaType < S > , errors : & mut Vec < RuleError > ,) where S : ScalarValue , { for (name , def) in var_defs . iter () { let raw_type_name = def . var_type . item . innermost_name () ; match schema . concrete_type_by_name (raw_type_name) { Some (t) if t . is_input () => { let ct = schema . make_type (& def . var_type . item) ; if def . var_type . item . is_non_null () && is_absent_or_null (values . get (name . item)) { errors . push (RuleError :: new (& format ! (r#"Variable "${}" of required type "{}" was not provided."# , name . item , def . var_type . item ,) , & [name . span . start] ,)) ; } else if let Some (v) = values . get (name . item) { errors . append (& mut unify_value (name . item , & name . span . start , v , & ct , schema , Path :: Root ,)) ; } } _ => unreachable ! (r#"Variable "${}" has invalid input type "{}" after document validation."# , name . item , def . var_type . item ,) , } } }
};
}
