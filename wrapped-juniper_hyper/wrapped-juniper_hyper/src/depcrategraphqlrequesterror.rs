// Generated macro for GraphQLRequestError (enum)
macro_rules! DepcrateGraphQLRequestError {
() => {
// Module: crate
// Provides: {"GraphQLRequestError"}
// Dependencies: {}
# [derive (Debug , Display , Error)] enum GraphQLRequestError < B : Body > { # [debug ("{_0:?}")] BodyHyper (B :: Error) , # [debug ("{_0:?}")] BodyUtf8 (FromUtf8Error) , # [debug ("{_0:?}")] BodyJSONError (SerdeError) , # [debug ("{_0:?}")] Variables (SerdeError) , # [debug ("{_0:?}")] Invalid (# [error (not (source))] String) , }
};
}
