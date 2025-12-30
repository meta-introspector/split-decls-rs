// Generated macro for receive_json (function)
macro_rules! Depcrate_httpreceive_json {
() => {
// Module: crate::http
// Provides: {"receive_json"}
// Dependencies: {}
# [doc = " Receive a GraphQL request from a body as JSON."] pub async fn receive_json (body : impl AsyncRead + Unpin) -> Result < Request , ParseRequestError > { receive_batch_json (body) . await ? . into_single () }
};
}
