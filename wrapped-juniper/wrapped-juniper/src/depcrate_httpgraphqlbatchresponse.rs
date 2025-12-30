// Generated macro for GraphQLBatchResponse (enum)
macro_rules! Depcrate_httpGraphQLBatchResponse {
() => {
// Module: crate::http
// Provides: {"GraphQLBatchResponse"}
// Dependencies: {}
# [doc = " Simple wrapper around the result (GraphQLResponse) from executing a GraphQLBatchRequest"] # [doc = ""] # [doc = " This struct implements Serialize, so you can simply serialize this"] # [doc = " to JSON and send it over the wire. use the `is_ok` to determine"] # [doc = " wheter to send a 200 or 400 HTTP status code."] # [derive (Serialize)] # [serde (untagged)] pub enum GraphQLBatchResponse < S = DefaultScalarValue > where S : ScalarValue , { # [doc = " Result of a single operation in a GraphQL request."] Single (GraphQLResponse < S >) , # [doc = " Result of a batch operation in a GraphQL request."] Batch (Vec < GraphQLResponse < S > >) , }
};
}
