// Generated macro for rzcobs_decode (function)
macro_rules! Depcrate_stream_rzcobsrzcobs_decode {
() => {
// Module: crate::stream::rzcobs
// Provides: {"rzcobs_decode"}
// Dependencies: {}
# [doc = " Decode a full message."] # [doc = ""] # [doc = " `data` must be a full rzCOBS encoded message. Decoding partial"] # [doc = " messages is not possible. `data` must NOT include any `0x00` separator byte."] fn rzcobs_decode (data : & [u8]) -> Result < Vec < u8 > , DecodeError > { let mut res = vec ! [] ; let mut data = data . iter () . rev () . cloned () ; while let Some (x) = data . next () { match x { 0 => return Err (DecodeError :: Malformed) , 0x01 ..= 0x7f => { for i in 0 .. 7 { if x & (1 << (6 - i)) == 0 { res . push (data . next () . ok_or (DecodeError :: Malformed) ?) ; } else { res . push (0) ; } } } 0x80 ..= 0xfe => { let n = (x & 0x7f) + 7 ; res . push (0) ; for _ in 0 .. n { res . push (data . next () . ok_or (DecodeError :: Malformed) ?) ; } } 0xff => { for _ in 0 .. 134 { res . push (data . next () . ok_or (DecodeError :: Malformed) ?) ; } } } } res . reverse () ; Ok (res) }
};
}
