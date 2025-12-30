// Generated macro for run_query (function)
macro_rules! Depcrate_executor_tests_variablesrun_query {
() => {
// Module: crate::executor_tests::variables
// Provides: {"run_query"}
// Dependencies: {}
async fn run_query < F > (query : & str , f : F) where F : Fn (& Object < DefaultScalarValue >) , { run_variable_query (query , graphql :: vars ! { } , f) . await ; }
};
}
