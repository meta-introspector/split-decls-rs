// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
# [cfg (not (any (feature = "graphql-transport-ws" , feature = "graphql-ws")))] compile_error ! (r#"at least one feature must be enabled (either "graphql-transport-ws" or "graphql-ws")"#) ;
};
}
