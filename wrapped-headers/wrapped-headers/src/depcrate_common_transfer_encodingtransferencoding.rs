// Generated macro for TransferEncoding (struct)
macro_rules! Depcrate_common_transfer_encodingTransferEncoding {
() => {
// Module: crate::common::transfer_encoding
// Provides: {"TransferEncoding"}
// Dependencies: {}
# [doc = " `Transfer-Encoding` header, defined in"] # [doc = " [RFC7230](https://datatracker.ietf.org/doc/html/rfc7230#section-3.3.1)"] # [doc = ""] # [doc = " The `Transfer-Encoding` header field lists the transfer coding names"] # [doc = " corresponding to the sequence of transfer codings that have been (or"] # [doc = " will be) applied to the payload body in order to form the message"] # [doc = " body."] # [doc = ""] # [doc = " Note that setting this header will *remove* any previously set"] # [doc = " `Content-Length` header, in accordance with"] # [doc = " [RFC7230](https://datatracker.ietf.org/doc/html/rfc7230#section-3.3.2):"] # [doc = ""] # [doc = " > A sender MUST NOT send a Content-Length header field in any message"] # [doc = " > that contains a Transfer-Encoding header field."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Transfer-Encoding = 1#transfer-coding"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `chunked`"] # [doc = " * `gzip, chunked`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::TransferEncoding;"] # [doc = ""] # [doc = " let transfer = TransferEncoding::chunked();"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct TransferEncoding (FlatCsv) ;
};
}
