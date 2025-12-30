// Generated macro for unify_scalar (function)
macro_rules! Depcrate_validation_input_valueunify_scalar {
() => {
// Module: crate::validation::input_value
// Provides: {"unify_scalar"}
// Dependencies: {}
fn unify_scalar < S > (var_name : & str , var_pos : & SourcePosition , value : & InputValue < S > , meta : & ScalarMeta < S > , path : & Path < '_ > ,) -> Vec < RuleError > where S : ScalarValue , { let mut errors : Vec < RuleError > = vec ! [] ; if let Err (e) = (meta . try_parse_fn) (value) { return vec ! [unification_error (var_name , var_pos , path , format ! ("Expected input scalar `{}`. Got: `{value}`. Details: {}" , meta . name , e . message () ,) ,)] ; } match * value { InputValue :: List (_) => errors . push (unification_error (var_name , var_pos , path , format ! (r#"Expected "{}", found list"# , meta . name) ,)) , InputValue :: Object (_) => errors . push (unification_error (var_name , var_pos , path , format ! (r#"Expected "{}", found object"# , meta . name) ,)) , _ => () , } errors }
};
}
