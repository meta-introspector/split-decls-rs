// Generated macro for ApolloTracing (struct)
macro_rules! Depcrate_extensions_apollo_tracingApolloTracing {
() => {
// Module: crate::extensions::apollo_tracing
// Provides: {"ApolloTracing"}
// Dependencies: {}
# [doc = " Apollo tracing extension for performance tracing"] # [doc = ""] # [doc = " Apollo Tracing works by including data in the extensions field of the"] # [doc = " GraphQL response, which is reserved by the GraphQL spec for extra"] # [doc = " information that a server wants to return. That way, you have access to"] # [doc = " performance traces alongside the data returned by your query. It's already"] # [doc = " supported by `Apollo Engine`, and we're excited to see what other kinds of"] # [doc = " integrations people can build on top of this format."] # [cfg_attr (docsrs , doc (cfg (feature = "apollo_tracing")))] pub struct ApolloTracing ;
};
}
