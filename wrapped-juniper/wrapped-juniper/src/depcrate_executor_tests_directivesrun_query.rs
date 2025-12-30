// Generated macro for run_query (function)
macro_rules! Depcrate_executor_tests_directivesrun_query {
() => {
// Module: crate::executor_tests::directives
// Provides: {"run_query"}
// Dependencies: {}
async fn run_query < F > (query : & str , f : F) where F : Fn (& Object < DefaultScalarValue >) , { run_variable_query (query , Variables :: new () , f) . await ; }
};
}
