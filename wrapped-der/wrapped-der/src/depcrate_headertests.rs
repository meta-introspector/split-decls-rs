// Generated macro for tests (module)
macro_rules! Depcrate_headertests {
() => {
// Module: crate::header
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Header ; use crate :: { Encode , Length , Reader , SliceReader , Tag , TagNumber } ; use hex_literal :: hex ; # [test] fn peek () { const EXAMPLE_MSG : & [u8] = & hex ! ("02012A00") ; let reader = SliceReader :: new (EXAMPLE_MSG) . expect ("slice to be valid length") ; assert_eq ! (reader . position () , Length :: ZERO) ; let header = Header :: peek (& reader) . expect ("peeked tag") ; assert_eq ! (header . tag () , Tag :: Integer) ; assert_eq ! (header . length () , Length :: ONE) ; assert_eq ! (reader . position () , Length :: ZERO) ; } # [test] fn peek_max_header () { const MAX_HEADER : [u8 ; 11] = hex ! ("BF8FFFFFFF7F 84FFFFFFFF") ; let reader = SliceReader :: new (& MAX_HEADER) . expect ("slice to be valid length") ; let header = Header :: peek (& reader) . expect ("peeked tag") ; assert_eq ! (header . tag , Tag :: ContextSpecific { constructed : true , number : TagNumber (0xFFFFFFFF) }) ; assert_eq ! (header . length () , Length :: new_usize (0xFFFFFFFF) . expect ("u32 to fit")) ; assert_eq ! (header . encoded_len () , Ok (Length :: new (11))) ; assert_eq ! (reader . position () , Length :: ZERO) ; } # [test] fn negative_peek_overlength_header () { const MAX_HEADER : [u8 ; 12] = hex ! ("BF8FFFFFFFFF7F 84FFFFFFFF") ; let reader = SliceReader :: new (& MAX_HEADER) . expect ("slice to be valid length") ; Header :: peek (& reader) . expect_err ("overlength error") ; } }
};
}
