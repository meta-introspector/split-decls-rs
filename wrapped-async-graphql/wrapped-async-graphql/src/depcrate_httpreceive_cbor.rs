// Generated macro for receive_cbor (function)
macro_rules! Depcrate_httpreceive_cbor {
() => {
// Module: crate::http
// Provides: {"receive_cbor"}
// Dependencies: {}
# [doc = " Receive a GraphQL request from a body as CBOR."] # [cfg (feature = "cbor")] # [cfg_attr (docsrs , doc (cfg (feature = "cbor")))] pub async fn receive_cbor (body : impl AsyncRead + Unpin) -> Result < BatchRequest , ParseRequestError > { receive_batch_cbor (body) . await ? . into_single () }
};
}
