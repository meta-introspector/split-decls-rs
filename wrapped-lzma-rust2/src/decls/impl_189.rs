macro_rules! deps {
    () => {
        Write!();
        Result!();
        LiteralEncoder!();
        LzmaEncoder!();
        State!();
        LzmaCoder!();
        LzEncoder!();
        LiteralSubEncoder!();
        LzmaEncData!();
        RangeEncoder!();
        LiteralCoder!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl LiteralEncoder { pub (crate) fn new (lc : u32 , lp : u32) -> Self { Self { coder : LiteralCoder :: new (lc , lp) , sub_encoders : vec ! [LiteralSubEncoder :: new () ; 1 << (lc + lp)] , } } pub (crate) fn reset (& mut self) { for ele in self . sub_encoders . iter_mut () { ele . reset () ; } } pub (crate) fn encode_init < W : Write > (& mut self , lz : & LzEncoder , data : & LzmaEncData , coder : & mut LzmaCoder , rc : & mut RangeEncoder < W > ,) -> crate :: Result < () > { debug_assert ! (data . read_ahead >= 0) ; self . sub_encoders [0] . encode (lz , data , coder , rc) } pub (crate) fn encode < W : Write > (& mut self , lz : & LzEncoder , data : & LzmaEncData , coder : & mut LzmaCoder , rc : & mut RangeEncoder < W > ,) -> crate :: Result < () > { debug_assert ! (data . read_ahead >= 0) ; let i = self . coder . get_sub_coder_index (lz . get_byte_backward (1 + data . read_ahead) as _ , (lz . get_pos () - data . read_ahead) as u32 ,) ; self . sub_encoders [i as usize] . encode (lz , data , coder , rc) } pub (crate) fn get_price (& self , encoder : & LzmaEncoder , cur_byte : u32 , match_byte : u32 , prev_byte : u32 , pos : u32 , state : & State ,) -> u32 { let mut price = RangeEncoder :: get_bit_price (encoder . coder . is_match [state . get () as usize] [(pos & encoder . coder . pos_mask) as usize] as _ , 0 ,) ; let i = self . coder . get_sub_coder_index (prev_byte , pos) as usize ; price += if state . is_literal () { self . sub_encoders [i] . get_normal_price (cur_byte) } else { self . sub_encoders [i] . get_matched_price (cur_byte , match_byte) } ; price } }
    };
}

impl_189!();