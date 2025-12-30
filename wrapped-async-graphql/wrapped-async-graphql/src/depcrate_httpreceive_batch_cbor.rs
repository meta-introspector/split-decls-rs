// Generated macro for receive_batch_cbor (function)
macro_rules! Depcrate_httpreceive_batch_cbor {
() => {
// Module: crate::http
// Provides: {"receive_batch_cbor"}
// Dependencies: {}
# [doc = " Receive a GraphQL batch request from a body as CBOR"] # [cfg (feature = "cbor")] # [cfg_attr (docsrs , doc (cfg (feature = "cbor")))] pub async fn receive_batch_cbor (body : impl AsyncRead + Unpin) -> Result < BatchRequest , ParseRequestError > { let mut data = Vec :: new () ; futures_util :: pin_mut ! (body) ; body . read_to_end (& mut data) . await . map_err (ParseRequestError :: Io) ? ; serde_cbor :: from_slice :: < BatchRequest > (& data) . map_err (| e | ParseRequestError :: InvalidRequest (Box :: new (e))) }
};
}
