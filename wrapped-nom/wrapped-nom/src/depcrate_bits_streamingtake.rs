// Generated macro for take (function)
macro_rules! Depcrate_bits_streamingtake {
() => {
// Module: crate::bits::streaming
// Provides: {"take"}
// Dependencies: {}
# [doc = " Generates a parser taking `count` bits"] pub fn take < I , O , C , E : ParseError < (I , usize) > > (count : C ,) -> impl Fn ((I , usize)) -> IResult < (I , usize) , O , E > where I : Input < Item = u8 > , C : ToUsize , O : From < u8 > + AddAssign + Shl < usize , Output = O > + Shr < usize , Output = O > , { let count = count . to_usize () ; move | (input , bit_offset) : (I , usize) | { if count == 0 { Ok (((input , bit_offset) , 0u8 . into ())) } else { let cnt = (count + bit_offset) . div (8) ; if input . input_len () * 8 < count + bit_offset { Err (Err :: Incomplete (Needed :: new (count))) } else { let mut acc : O = 0_u8 . into () ; let mut offset : usize = bit_offset ; let mut remaining : usize = count ; let mut end_offset : usize = 0 ; for byte in input . iter_elements () . take (cnt + 1) { if remaining == 0 { break ; } let val : O = if offset == 0 { byte . into () } else { ((byte << offset) >> offset) . into () } ; if remaining < 8 - offset { acc += val >> (8 - offset - remaining) ; end_offset = remaining + offset ; break ; } else { acc += val << (remaining - (8 - offset)) ; remaining -= 8 - offset ; offset = 0 ; } } Ok (((input . take_from (cnt) , end_offset) , acc)) } } } }
};
}
