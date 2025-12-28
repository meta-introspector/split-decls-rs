macro_rules! deps {
    () => {
        RangeDecoder!();
        LzDecoder!();
        LiteralSubDecoder!();
        LzmaCoder!();
        LiteralSubCoder!();
        Result!();
        RangeReader!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl LiteralSubDecoder { fn new () -> Self { Self { coder : LiteralSubCoder :: new () , } } pub (crate) fn decode < R : RangeReader > (& mut self , coder : & mut LzmaCoder , lz : & mut LzDecoder , rc : & mut RangeDecoder < R > ,) -> crate :: Result < () > { let mut symbol : u32 = 1 ; let liter = coder . state . is_literal () ; if liter { loop { let b = rc . decode_bit (& mut self . coder . probs [symbol as usize]) as u32 ; symbol = (symbol << 1) | b ; if symbol >= 0x100 { break ; } } } else { let r = coder . reps [0] ; let mut match_byte = lz . get_byte (r as usize) as u32 ; let mut offset = 0x100 ; let mut match_bit ; let mut bit ; loop { match_byte <<= 1 ; match_bit = match_byte & offset ; bit = rc . decode_bit (& mut self . coder . probs [(offset + match_bit + symbol) as usize]) as u32 ; symbol = (symbol << 1) | bit ; offset &= (0u32 . wrapping_sub (bit)) ^ ! match_bit ; if symbol >= 0x100 { break ; } } } lz . put_byte (symbol as u8) ; coder . state . update_literal () ; Ok (()) } }
    };
}

impl_6!()