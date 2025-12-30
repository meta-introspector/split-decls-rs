// Generated macro for huffman_encoding_len (function)
macro_rules! Depcratehuffman_encoding_len {
() => {
// Module: crate
// Provides: {"huffman_encoding_len"}
// Dependencies: {}
# [doc = " Returns how long the Huffman encoding of the given buffer will be."] # [doc = ""] # [doc = " The Huffman code implemented is the one defined for HPACK (RFC7541)."] # [cfg (feature = "huffman_hpack")] pub fn huffman_encoding_len < const LOWER_CASE : bool > (src : & [u8]) -> Result < usize > { use self :: huffman_table :: ENCODE_TABLE ; let mut bits : usize = 0 ; for & b in src { let b = if LOWER_CASE { b . to_ascii_lowercase () } else { b } ; let (nbits , _) = ENCODE_TABLE [b as usize] ; bits += nbits ; } let mut len = bits / 8 ; if bits & 7 != 0 { len += 1 ; } if len > src . len () { return Err (BufferTooShortError) ; } Ok (len) }
};
}
