// Generated macro for JuniperResponse (struct)
macro_rules! Depcrate_responseJuniperResponse {
() => {
// Module: crate::response
// Provides: {"JuniperResponse"}
// Dependencies: {}
# [doc = " Wrapper around a [`GraphQLBatchResponse`], implementing [`IntoResponse`], so it can be returned"] # [doc = " from [`axum`] handlers."] pub struct JuniperResponse < S = DefaultScalarValue > (pub GraphQLBatchResponse < S >) where S : ScalarValue ;
};
}
