// Generated macro for Alphabet (trait)
macro_rules! Depcrate_alphabetAlphabet {
() => {
// Module: crate::alphabet
// Provides: {"Alphabet"}
// Dependencies: {}
# [doc = " Core encoder/decoder functions for a particular Base64 alphabet"] pub trait Alphabet : 'static + Copy + Debug + Eq + Send + Sized + Sync { # [doc = " First character in this Base64 alphabet"] const BASE : u8 ; # [doc = " Decoder passes"] const DECODER : & 'static [DecodeStep] ; # [doc = " Encoder passes"] const ENCODER : & 'static [EncodeStep] ; # [doc = " Is this encoding padded?"] const PADDED : bool ; # [doc = " Use bitwise operators instead of table-lookups to turn 5-bit integers"] # [doc = " into 8-bit integers."] fn decode_5bits (byte : u8) -> i16 { let src = byte as i16 ; let mut ret : i16 = - 1 ; for DecodeStep (range , offset) in Self :: DECODER { let start = * range . start () as i16 - 1 ; let end = * range . end () as i16 + 1 ; ret += (((start - src) & (src - end)) >> 8) & (src + * offset) ; } ret } # [doc = " Use bitwise operators instead of table-lookups to turn 8-bit integers"] # [doc = " into 5-bit integers."] fn encode_5bits (byte : u8) -> u8 { let src = byte as i16 ; let mut diff = src + Self :: BASE as i16 ; for & EncodeStep (threshold , offset) in Self :: ENCODER { diff -= ((threshold as i16 - src) >> 8) & offset ; } diff as u8 } }
};
}
