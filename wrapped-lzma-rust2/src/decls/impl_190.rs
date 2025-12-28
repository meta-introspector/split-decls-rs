macro_rules! deps {
    () => {
        Result!();
        RangeEncoder!();
        LzEncoder!();
        Write!();
        LiteralSubEncoder!();
        LiteralSubCoder!();
        LzmaEncData!();
        LzmaCoder!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl LiteralSubEncoder { fn new () -> Self { Self { coder : LiteralSubCoder :: new () , } } fn reset (& mut self) { self . coder . reset () } fn encode < W : Write > (& mut self , lz : & LzEncoder , data : & LzmaEncData , coder : & mut LzmaCoder , rc : & mut RangeEncoder < W > ,) -> crate :: Result < () > { let mut symbol = lz . get_byte_backward (data . read_ahead) as u32 | 0x100 ; if coder . state . is_literal () { let mut subencoder_index ; let mut bit ; loop { subencoder_index = symbol >> 8 ; bit = (symbol >> 7) & 1 ; rc . encode_bit (& mut self . coder . probs , subencoder_index as _ , bit as _) ? ; symbol <<= 1 ; if symbol >= 0x10000 { break ; } } } else { let mut match_byte = lz . get_byte_backward (coder . reps [0] + 1 + data . read_ahead) as u32 ; let mut offset = 0x100 ; let mut subencoder_index ; let mut match_bit ; let mut bit ; loop { match_byte <<= 1 ; match_bit = match_byte & offset ; subencoder_index = offset + match_bit + (symbol >> 8) ; bit = (symbol >> 7) & 1 ; rc . encode_bit (& mut self . coder . probs , subencoder_index as _ , bit) ? ; symbol <<= 1 ; offset &= ! (match_byte ^ symbol) ; if symbol >= 0x10000 { break ; } } } coder . state . update_literal () ; Ok (()) } fn get_normal_price (& self , symbol : u32) -> u32 { let mut price : u32 = 0 ; let mut subencoder_index ; let mut bit ; let mut symbol = symbol | 0x100 ; loop { subencoder_index = symbol >> 8 ; bit = (symbol >> 7) & 1 ; price += RangeEncoder :: get_bit_price (self . coder . probs [subencoder_index as usize] as _ , bit as _ ,) ; symbol <<= 1 ; if symbol >= (0x100 << 8) { break ; } } price } fn get_matched_price (& self , symbol : u32 , mut match_byte : u32) -> u32 { let mut price = 0 ; let mut offset = 0x100 ; let mut subencoder_index ; let mut match_bit ; let mut bit ; let mut symbol = symbol | 0x100 ; loop { match_byte <<= 1 ; match_bit = match_byte & offset ; subencoder_index = offset + match_bit + (symbol >> 8) ; bit = (symbol >> 7) & 1 ; price += RangeEncoder :: get_bit_price (self . coder . probs [subencoder_index as usize] as _ , bit as _ ,) ; symbol <<= 1 ; offset &= ! (match_byte ^ symbol) ; if symbol >= (0x100 << 8) { break ; } } price } }
    };
}

impl_190!()