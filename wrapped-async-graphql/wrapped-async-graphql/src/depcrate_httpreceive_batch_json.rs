// Generated macro for receive_batch_json (function)
macro_rules! Depcrate_httpreceive_batch_json {
() => {
// Module: crate::http
// Provides: {"receive_batch_json"}
// Dependencies: {}
# [doc = " Receive a GraphQL batch request from a body as JSON."] pub async fn receive_batch_json (body : impl AsyncRead + Unpin) -> Result < BatchRequest , ParseRequestError > { let mut data = Vec :: new () ; futures_util :: pin_mut ! (body) ; body . read_to_end (& mut data) . await . map_err (ParseRequestError :: Io) ? ; serde_json :: from_slice :: < BatchRequest > (& data) . map_err (| e | ParseRequestError :: InvalidRequest (Box :: new (e))) }
};
}
