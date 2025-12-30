// Generated macro for BatchRequest (enum)
macro_rules! Depcrate_requestBatchRequest {
() => {
// Module: crate::request
// Provides: {"BatchRequest"}
// Dependencies: {}
# [doc = " Batch support for GraphQL requests, which is either a single query, or an"] # [doc = " array of queries"] # [doc = ""] # [doc = " **Reference:** <https://www.apollographql.com/blog/batching-client-graphql-queries-a685f5bcd41b/>"] # [derive (Debug , Deserialize)] # [serde (untagged)] # [allow (clippy :: large_enum_variant)] pub enum BatchRequest { # [doc = " Single query"] Single (Request) , # [doc = " Non-empty array of queries"] # [serde (deserialize_with = "deserialize_non_empty_vec")] Batch (Vec < Request >) , }
};
}
