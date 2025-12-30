// Generated macro for invalid_err (function)
macro_rules! Depcrateinvalid_err {
() => {
// Module: crate
// Provides: {"invalid_err"}
// Dependencies: {}
fn invalid_err < B : Body > (parameter_name : & str) -> GraphQLRequestError < B > { GraphQLRequestError :: Invalid (format ! ("`{parameter_name}` parameter is specified multiple times" ,)) }
};
}
