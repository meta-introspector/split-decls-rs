// Generated macro for impl_1490 (impl)
macro_rules! Depcrate_executor_tests_variablesimpl_1490 {
() => {
// Module: crate::executor_tests::variables
// Provides: {"impl_1490"}
// Dependencies: {}
# [graphql_object] impl TestType { fn field_with_object_input (input : Option < TestInputObject >) -> String { format ! ("{input:?}") } fn field_with_nullable_string_input (input : Option < String >) -> String { format ! ("{input:?}") } fn field_with_non_nullable_string_input (input : String) -> String { format ! ("{input:?}") } fn field_with_default_argument_value (# [graphql (default = "Hello World")] input : String ,) -> String { format ! ("{input:?}") } fn nullable_field_with_default_argument_value (# [graphql (default = "Hello World" . to_owned ())] input : Option < String > ,) -> String { format ! ("{input:?}") } fn field_with_nested_object_input (input : Option < TestNestedInputObject >) -> String { format ! ("{input:?}") } fn list (input : Option < Vec < Option < String > > >) -> String { format ! ("{input:?}") } fn nn_list (input : Vec < Option < String > >) -> String { format ! ("{input:?}") } fn list_nn (input : Option < Vec < String > >) -> String { format ! ("{input:?}") } fn nn_list_nn (input : Vec < String >) -> String { format ! ("{input:?}") } fn example_input (arg : ExampleInputObject) -> String { format ! ("a: {:?}, b: {:?}" , arg . a , arg . b) } fn input_with_defaults (arg : InputWithDefaults) -> String { format ! ("a: {:?}" , arg . a) } fn integer_input (value : i32) -> String { format ! ("value: {value}") } fn float_input (value : f64) -> String { format ! ("value: {value}") } }
};
}
