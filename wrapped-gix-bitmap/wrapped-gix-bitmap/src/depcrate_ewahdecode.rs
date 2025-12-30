// Generated macro for decode (function)
macro_rules! Depcrate_ewahdecode {
() => {
// Module: crate::ewah
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Decode `data` as EWAH bitmap."] pub fn decode (data : & [u8]) -> Result < (Vec , & [u8]) , decode :: Error > { use self :: decode :: Error ; use crate :: decode ; let (num_bits , data) = decode :: u32 (data) . ok_or (Error :: Corrupt { message : "eof reading amount of bits" , }) ? ; let (len , data) = decode :: u32 (data) . ok_or (Error :: Corrupt { message : "eof reading chunk length" , }) ? ; let len = len as usize ; let (mut bits , data) = data . split_at_checked (len * std :: mem :: size_of :: < u64 > ()) . ok_or (Error :: Corrupt { message : "eof while reading bit data" , }) ? ; let mut buf = std :: vec :: Vec :: < u64 > :: with_capacity (len) ; for _ in 0 .. len { let (bit_num , rest) = bits . split_at (std :: mem :: size_of :: < u64 > ()) ; bits = rest ; buf . push (u64 :: from_be_bytes (bit_num . try_into () . unwrap ())) ; } let (rlw , data) = decode :: u32 (data) . ok_or (Error :: Corrupt { message : "eof while reading run length width" , }) ? ; Ok ((Vec { num_bits , bits : buf , rlw : rlw . into () , } , data ,)) }
};
}
