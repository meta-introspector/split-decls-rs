// Generated macro for default_name_input_value (function)
macro_rules! Depcrate_executor_tests_introspection_input_objectdefault_name_input_value {
() => {
// Module: crate::executor_tests::introspection::input_object
// Provides: {"default_name_input_value"}
// Dependencies: {}
# [test] fn default_name_input_value () { let iv : InputValue = graphql :: input_value ! ({ "fieldOne" : "number one" , "fieldTwo" : "number two" , }) ; let dv = DefaultName :: from_input_value (& iv) ; assert ! (dv . is_ok () , "error: {}" , dv . unwrap_err () . message ()) ; let dv = dv . unwrap () ; assert_eq ! (dv . field_one , "number one") ; assert_eq ! (dv . field_two , "number two") ; }
};
}
