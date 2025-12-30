// Generated macro for unify_value (function)
macro_rules! Depcrate_validation_input_valueunify_value {
() => {
// Module: crate::validation::input_value
// Provides: {"unify_value"}
// Dependencies: {}
fn unify_value < 'a , S > (var_name : & str , var_pos : & SourcePosition , value : & InputValue < S > , meta_type : & TypeType < 'a , S > , schema : & SchemaType < S > , path : Path < 'a > ,) -> Vec < RuleError > where S : ScalarValue , { let mut errors : Vec < RuleError > = vec ! [] ; match * meta_type { TypeType :: NonNull (ref inner) => { if value . is_null () { errors . push (unification_error (var_name , var_pos , & path , format ! (r#"Expected "{meta_type}", found null"#) ,)) ; } else { errors . append (& mut unify_value (var_name , var_pos , value , inner , schema , path ,)) ; } } TypeType :: List (ref inner , expected_size) => { if value . is_null () { return errors ; } match value . to_list_value () { Some (l) => { if let Some (expected) = expected_size { if l . len () != expected { errors . push (unification_error (var_name , var_pos , & path , format ! ("Expected list of {expected} elements, \
                                     found {} elements" , l . len () ,) ,)) ; } } for (i , v) in l . iter () . enumerate () { errors . append (& mut unify_value (var_name , var_pos , v , inner , schema , Path :: ArrayElement (i , & path) ,)) ; } } _ => errors . append (& mut unify_value (var_name , var_pos , value , inner , schema , path ,)) , } } TypeType :: Concrete (mt) => { if value . is_null () { return errors ; } match * mt { MetaType :: Scalar (ref sm) => { errors . append (& mut unify_scalar (var_name , var_pos , value , sm , & path)) } MetaType :: Enum (ref em) => { errors . append (& mut unify_enum (var_name , var_pos , value , em , & path)) } MetaType :: InputObject (ref iom) => { let mut e = unify_input_object (var_name , var_pos , value , iom , schema , & path) ; if e . is_empty () { if let Err (e) = (iom . try_parse_fn) (value) { errors . push (unification_error (var_name , var_pos , & path , format ! ("Expected input of type `{}`. \
                                     Got: `{value}`. \
                                     Details: {}" , iom . name , e . message () ,) ,)) ; } } else { errors . append (& mut e) ; } } _ => panic ! ("Can't unify non-input concrete type") , } } } errors }
};
}
