// Generated macro for post_json_extractor (function)
macro_rules! Depcratepost_json_extractor {
() => {
// Module: crate
// Provides: {"post_json_extractor"}
// Dependencies: {}
# [doc = " Extracts a [`GraphQLBatchRequest`] from a POST `application/json` HTTP request."] fn post_json_extractor < S > () -> impl Filter < Extract = (GraphQLBatchRequest < S > ,) , Error = Rejection > + Clone + Send where S : ScalarValue + Send , { warp :: post () . and (body :: json ()) }
};
}
