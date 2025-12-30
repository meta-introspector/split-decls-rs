// Generated macro for decode_sequence (function)
macro_rules! Depcrate_documentdecode_sequence {
() => {
// Module: crate::document
// Provides: {"decode_sequence"}
// Dependencies: {}
# [doc = " Attempt to decode a ASN.1 `SEQUENCE` from the given decoder, returning the"] # [doc = " entire sequence including the header."] fn decode_sequence < 'a > (decoder : & mut SliceReader < 'a >) -> Result < & 'a [u8] , Error > { let header = Header :: peek (decoder) ? ; header . tag () . assert_eq (Tag :: Sequence) ? ; let len = (header . encoded_len () ? + header . length ()) ? ; decoder . read_slice (len) }
};
}
