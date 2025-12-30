// Generated macro for GraphQLBatchRequest (enum)
macro_rules! Depcrate_httpGraphQLBatchRequest {
() => {
// Module: crate::http
// Provides: {"GraphQLBatchRequest"}
// Dependencies: {}
# [doc = " Simple wrapper around GraphQLRequest to allow the handling of Batch requests."] # [derive (Debug , Deserialize , PartialEq)] # [serde (untagged)] # [serde (bound = "InputValue<S>: Deserialize<'de>")] pub enum GraphQLBatchRequest < S = DefaultScalarValue > where S : ScalarValue , { # [doc = " A single operation request."] Single (GraphQLRequest < S >) , # [doc = " A batch operation request."] # [doc = ""] # [doc = " Empty batch is considered as invalid value, so cannot be deserialized."] # [serde (deserialize_with = "deserialize_non_empty_batch")] Batch (Vec < GraphQLRequest < S > >) , }
};
}
