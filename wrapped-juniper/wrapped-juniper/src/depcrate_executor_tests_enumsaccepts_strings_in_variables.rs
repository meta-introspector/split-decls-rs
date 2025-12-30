// Generated macro for accepts_strings_in_variables (function)
macro_rules! Depcrate_executor_tests_enumsaccepts_strings_in_variables {
() => {
// Module: crate::executor_tests::enums
// Provides: {"accepts_strings_in_variables"}
// Dependencies: {}
# [tokio :: test] async fn accepts_strings_in_variables () { run_variable_query ("query q($color: Color!) { toString(color: $color) }" , graphql :: vars ! { "color" : "RED" } , | result | { assert_eq ! (result . get_field_value ("toString") , Some (& graphql :: value ! ("Color::Red")) ,) ; } ,) . await ; }
};
}
