// Generated macro for receive_batch_body_no_multipart (function)
macro_rules! Depcrate_httpreceive_batch_body_no_multipart {
() => {
// Module: crate::http
// Provides: {"receive_batch_body_no_multipart"}
// Dependencies: {}
# [doc = " Receives a GraphQL query which is either cbor or json but NOT multipart"] # [doc = " This method is only to avoid recursive calls with [``receive_batch_body``]"] # [doc = " and [``multipart::receive_batch_multipart``]"] pub (super) async fn receive_batch_body_no_multipart (content_type : & mime :: Mime , body : impl AsyncRead + Send ,) -> Result < BatchRequest , ParseRequestError > { assert_ne ! (content_type . type_ () , mime :: MULTIPART , "received multipart") ; match (content_type . type_ () , content_type . subtype ()) { # [cfg (feature = "cbor")] (mime :: OCTET_STREAM , _) | (mime :: APPLICATION , mime :: OCTET_STREAM) => { receive_batch_cbor (body) . await } _ => receive_batch_json (body) . await , } }
};
}
