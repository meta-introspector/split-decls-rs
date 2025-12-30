// Generated macro for encode_header_block (function)
macro_rules! Depcrateencode_header_block {
() => {
// Module: crate
// Provides: {"encode_header_block"}
// Dependencies: {}
fn encode_header_block (headers : & [quiche :: h3 :: Header] ,) -> std :: result :: Result < Vec < u8 > , String > { let mut encoder = quiche :: h3 :: qpack :: Encoder :: new () ; let headers_len = headers . iter () . fold (0 , | acc , h | acc + h . value () . len () + h . name () . len () + 32) ; let mut header_block = vec ! [0 ; headers_len] ; let len = encoder . encode (headers , & mut header_block) . map_err (| _ | "Internal Error") ? ; header_block . truncate (len) ; Ok (header_block) }
};
}
