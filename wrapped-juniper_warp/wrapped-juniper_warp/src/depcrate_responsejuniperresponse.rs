// Generated macro for JuniperResponse (struct)
macro_rules! Depcrate_responseJuniperResponse {
() => {
// Module: crate::response
// Provides: {"JuniperResponse"}
// Dependencies: {}
# [doc = " Wrapper around a [`GraphQLBatchResponse`], implementing [`warp::Reply`], so it can be returned"] # [doc = " from [`warp`] handlers."] pub (crate) struct JuniperResponse < S = DefaultScalarValue > (pub (crate) GraphQLBatchResponse < S >) where S : ScalarValue ;
};
}
