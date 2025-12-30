// Generated macro for post_graphql_extractor (function)
macro_rules! Depcratepost_graphql_extractor {
() => {
// Module: crate
// Provides: {"post_graphql_extractor"}
// Dependencies: {}
# [doc = " Extracts a [`GraphQLBatchRequest`] from a POST `application/graphql` HTTP request."] fn post_graphql_extractor < S > () -> impl Filter < Extract = (GraphQLBatchRequest < S > ,) , Error = Rejection > + Clone + Send where S : ScalarValue + Send , { warp :: post () . and (body :: bytes ()) . and_then (async | body : Bytes | { let query = str :: from_utf8 (body . as_ref ()) . map_err (| e | reject :: custom (FilterError :: NonUtf8Body (e))) ? ; let req = GraphQLRequest :: new (query . into () , None , None) ; Ok :: < GraphQLBatchRequest < S > , Rejection > (GraphQLBatchRequest :: Single (req)) }) }
};
}
