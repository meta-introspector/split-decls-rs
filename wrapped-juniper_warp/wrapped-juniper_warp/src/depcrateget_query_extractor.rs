// Generated macro for get_query_extractor (function)
macro_rules! Depcrateget_query_extractor {
() => {
// Module: crate
// Provides: {"get_query_extractor"}
// Dependencies: {}
# [doc = " Extracts a [`GraphQLBatchRequest`] from a GET HTTP request."] fn get_query_extractor < S > () -> impl Filter < Extract = (GraphQLBatchRequest < S > ,) , Error = Rejection > + Clone + Send where S : ScalarValue + Send , { warp :: get () . and (query :: query ()) . and_then (async | mut qry : HashMap < String , String > | { let req = GraphQLRequest :: new (qry . remove ("query") . ok_or_else (| | reject :: custom (FilterError :: MissingPathQuery)) ? , qry . remove ("operation_name") , qry . remove ("variables") . map (| vs | serde_json :: from_str (& vs)) . transpose () . map_err (| e | reject :: custom (FilterError :: InvalidPathVariables (e))) ? ,) ; Ok :: < GraphQLBatchRequest < S > , Rejection > (GraphQLBatchRequest :: Single (req)) }) }
};
}
