// Generated macro for Error (enum)
macro_rules! Depcrate_h3_qpackError {
() => {
// Module: crate::h3::qpack
// Provides: {"Error"}
// Dependencies: {}
# [doc = " A QPACK error."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Error { # [doc = " The provided buffer is too short."] BufferTooShort , # [doc = " The QPACK header block's huffman encoding is invalid."] InvalidHuffmanEncoding , # [doc = " The QPACK static table index provided doesn't exist."] InvalidStaticTableIndex , # [doc = " The decoded QPACK header name or value is not valid."] InvalidHeaderValue , # [doc = " The decoded header list exceeded the size limit."] HeaderListTooLarge , }
};
}
