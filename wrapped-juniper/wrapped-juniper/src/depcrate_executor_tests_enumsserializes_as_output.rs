// Generated macro for serializes_as_output (function)
macro_rules! Depcrate_executor_tests_enumsserializes_as_output {
() => {
// Module: crate::executor_tests::enums
// Provides: {"serializes_as_output"}
// Dependencies: {}
# [tokio :: test] async fn serializes_as_output () { run_query ("{ aColor }" , | result | { assert_eq ! (result . get_field_value ("aColor") , Some (& graphql :: value ! ("RED")) ,) ; }) . await ; }
};
}
