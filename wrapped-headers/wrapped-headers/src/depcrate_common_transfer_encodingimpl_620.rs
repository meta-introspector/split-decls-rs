// Generated macro for impl_620 (impl)
macro_rules! Depcrate_common_transfer_encodingimpl_620 {
() => {
// Module: crate::common::transfer_encoding
// Provides: {"impl_620"}
// Dependencies: {}
impl TransferEncoding { # [doc = " Constructor for the most common Transfer-Encoding, `chunked`."] pub fn chunked () -> TransferEncoding { TransferEncoding (HeaderValue :: from_static ("chunked") . into ()) } # [doc = " Returns whether this ends with the `chunked` encoding."] pub fn is_chunked (& self) -> bool { self . 0 . value . to_str () . map (| s | { s . split (',') . next_back () . map (| encoding | encoding . trim () == "chunked") . expect ("split always has at least 1 item") }) . unwrap_or (false) } }
};
}
