// Generated macro for unify_input_object (function)
macro_rules! Depcrate_validation_input_valueunify_input_object {
() => {
// Module: crate::validation::input_value
// Provides: {"unify_input_object"}
// Dependencies: {}
fn unify_input_object < S > (var_name : & str , var_pos : & SourcePosition , value : & InputValue < S > , meta : & InputObjectMeta < S > , schema : & SchemaType < S > , path : & Path < '_ > ,) -> Vec < RuleError > where S : ScalarValue , { let mut errors : Vec < RuleError > = vec ! [] ; if let Some (ref obj) = value . to_object_value () { let mut keys = obj . keys () . collect :: < HashSet < & & str > > () ; for input_field in & meta . input_fields { let mut has_value = false ; keys . remove (& input_field . name . as_str ()) ; if let Some (value) = obj . get (input_field . name . as_str ()) { if ! value . is_null () { has_value = true ; errors . append (& mut unify_value (var_name , var_pos , value , & schema . make_type (& input_field . arg_type) , schema , Path :: ObjectField (& input_field . name , path) ,)) ; } } if ! has_value && input_field . arg_type . is_non_null () { errors . push (unification_error (var_name , var_pos , & Path :: ObjectField (& input_field . name , path) , format ! (r#"Expected "{}", found null"# , input_field . arg_type) ,)) ; } } for key in keys { errors . push (unification_error (var_name , var_pos , & Path :: ObjectField (key , path) , "Unknown field" ,)) ; } } else { errors . push (unification_error (var_name , var_pos , path , format ! (r#"Expected "{}", found not an object"# , meta . name) ,)) ; } errors }
};
}
