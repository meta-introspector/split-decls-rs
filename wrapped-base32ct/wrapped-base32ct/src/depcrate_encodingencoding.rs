// Generated macro for Encoding (trait)
macro_rules! Depcrate_encodingEncoding {
() => {
// Module: crate::encoding
// Provides: {"Encoding"}
// Dependencies: {}
# [doc = " Core encoder/decoder functions for a particular Base32 alphabet"] pub trait Encoding : Alphabet { # [doc = " Decode a Base32-encoded string into the provided output buffer,"] # [doc = " returning a slice containing the decoded data."] fn decode (src : impl AsRef < [u8] > , dst : & mut [u8]) -> Result < & [u8] > ; # [doc = " Decode a Base32 string into a byte vector."] # [cfg (feature = "alloc")] fn decode_vec (input : & str) -> Result < Vec < u8 > > ; # [doc = " Encode the input byte slice as Base32."] # [doc = ""] # [doc = " Writes the result into the provided destination slice, returning an"] # [doc = " ASCII-encoded Base32 string value."] fn encode < 'a > (src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a str > ; # [doc = " Encode input byte slice into a [`String`] containing Base32."] # [cfg (feature = "alloc")] fn encode_string (input : & [u8]) -> String ; # [doc = " Get the length of Base32 produced by encoding the given bytes."] fn encoded_len (bytes : & [u8]) -> usize ; }
};
}
