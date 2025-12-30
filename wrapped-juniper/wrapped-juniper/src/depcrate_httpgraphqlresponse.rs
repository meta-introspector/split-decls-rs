// Generated macro for GraphQLResponse (struct)
macro_rules! Depcrate_httpGraphQLResponse {
() => {
// Module: crate::http
// Provides: {"GraphQLResponse"}
// Dependencies: {}
# [doc = " Simple wrapper around the result from executing a GraphQL query"] # [doc = ""] # [doc = " This struct implements Serialize, so you can simply serialize this"] # [doc = " to JSON and send it over the wire. Use the `is_ok` method to determine"] # [doc = " whether to send a 200 or 400 HTTP status code."] # [derive (Clone , Debug , PartialEq)] pub struct GraphQLResponse < S = DefaultScalarValue > (Result < (Value < S > , Vec < ExecutionError < S > >) , GraphQLError > ,) ;
};
}
