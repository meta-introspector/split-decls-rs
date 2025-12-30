// Generated macro for unify_enum (function)
macro_rules! Depcrate_validation_input_valueunify_enum {
() => {
// Module: crate::validation::input_value
// Provides: {"unify_enum"}
// Dependencies: {}
fn unify_enum < S > (var_name : & str , var_pos : & SourcePosition , value : & InputValue < S > , meta : & EnumMeta < S > , path : & Path < '_ > ,) -> Vec < RuleError > where S : ScalarValue , { let mut errors : Vec < RuleError > = vec ! [] ; match value { InputValue :: Scalar (scalar) if scalar . try_as_str () . is_some () => { if let Some (name) = scalar . try_as_str () { if ! meta . values . iter () . any (| ev | ev . name == * name) { errors . push (unification_error (var_name , var_pos , path , format ! (r#"Invalid value for enum "{}""# , meta . name) ,)) } } } InputValue :: Enum (name) => { if ! meta . values . iter () . any (| ev | & ev . name == name) { errors . push (unification_error (var_name , var_pos , path , format ! (r#"Invalid value for enum "{}""# , meta . name) ,)) } } _ => errors . push (unification_error (var_name , var_pos , path , format ! (r#"Expected "{}", found not a string or enum"# , meta . name) ,)) , } errors }
};
}
