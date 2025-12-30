// Generated macro for validate_last_block (function)
macro_rules! Depcrate_encodingvalidate_last_block {
() => {
// Module: crate::encoding
// Provides: {"validate_last_block"}
// Dependencies: {}
# [doc = " Validate that the last block of the decoded data round-trips back to the"] # [doc = " encoded data."] fn validate_last_block < T : Alphabet > (encoded : & [u8] , decoded : & [u8]) -> Result < () , Error > { if encoded . is_empty () && decoded . is_empty () { return Ok (()) ; } # [allow (clippy :: arithmetic_side_effects)] fn last_block_start (bytes : & [u8] , block_size : usize) -> usize { (bytes . len () . saturating_sub (1) / block_size) * block_size } let enc_block = encoded . get (last_block_start (encoded , 4) ..) . ok_or (Error :: InvalidEncoding) ? ; let dec_block = decoded . get (last_block_start (decoded , 3) ..) . ok_or (Error :: InvalidEncoding) ? ; let mut buf = [0u8 ; 4] ; let block = T :: encode (dec_block , & mut buf) ? ; if block . as_bytes () . iter () . zip (enc_block . iter ()) . fold (0 , | acc , (a , b) | acc | (a ^ b)) == 0 { Ok (()) } else { Err (Error :: InvalidEncoding) } }
};
}
