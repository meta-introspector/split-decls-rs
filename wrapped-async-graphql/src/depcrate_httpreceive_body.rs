// Generated macro for receive_body (function)
macro_rules! Depcrate_httpreceive_body {
() => {
// Module: crate::http
// Provides: {"receive_body"}
// Dependencies: {}
# [doc = " Receive a GraphQL request from a content type and body."] pub async fn receive_body (content_type : Option < impl AsRef < str > > , body : impl AsyncRead + Send , opts : MultipartOptions ,) -> Result < Request , ParseRequestError > { receive_batch_body (content_type , body , opts) . await ? . into_single () }
};
}
