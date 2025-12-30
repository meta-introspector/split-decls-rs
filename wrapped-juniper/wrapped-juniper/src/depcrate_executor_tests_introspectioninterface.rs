// Generated macro for Interface (trait)
macro_rules! Depcrate_executor_tests_introspectionInterface {
() => {
// Module: crate::executor_tests::introspection
// Provides: {"Interface"}
// Dependencies: {}
# [doc = " A sample interface"] # [expect (dead_code , reason = "GraphQL schema testing")] # [graphql_interface] # [graphql (name = "SampleInterface" , for = Root)] trait Interface { # [doc = " A sample field in the interface"] fn sample_enum (& self) -> Sample ; }
};
}
