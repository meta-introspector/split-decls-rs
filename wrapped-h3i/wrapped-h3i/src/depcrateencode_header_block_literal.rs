// Generated macro for encode_header_block_literal (function)
macro_rules! Depcrateencode_header_block_literal {
() => {
// Module: crate
// Provides: {"encode_header_block_literal"}
// Dependencies: {}
# [doc = " Encodes a header block literally. Unlike [`encode_header_block`],"] # [doc = " this function encodes all the headers exactly as provided. This"] # [doc = " means it does not use the huffman lookup table, nor does it convert"] # [doc = " the header names to lowercase before encoding."] fn encode_header_block_literal (headers : & [quiche :: h3 :: Header] ,) -> std :: result :: Result < Vec < u8 > , String > { let headers_len = headers . iter () . fold (0 , | acc , h | acc + h . value () . len () + h . name () . len () + 32) ; let mut header_block = vec ! [0 ; headers_len] ; let mut b = octets :: OctetsMut :: with_slice (& mut header_block) ; encode_int (0 , 0 , 8 , & mut b) . map_err (| e | format ! ("{e:?}")) ? ; encode_int (0 , 0 , 7 , & mut b) . map_err (| e | format ! ("{e:?}")) ? ; for h in headers { encode_str :: < false > (h . name () , LITERAL , 3 , & mut b) . map_err (| e | format ! ("{e:?}")) ? ; encode_str :: < false > (h . value () , 0 , 7 , & mut b) . map_err (| e | format ! ("{e:?}")) ? ; } let len = b . off () ; header_block . truncate (len) ; Ok (header_block) }
};
}
