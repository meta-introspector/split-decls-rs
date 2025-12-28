macro_rules! deps {
    () => {
        Result!();
        HuffmanOxide!();
        Rle!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl Rle { fn prev_code_size (& mut self , packed_code_sizes : & mut [u8] , packed_pos : & mut usize , h : & mut HuffmanOxide ,) -> Result < () > { let mut write = | buf | write (buf , packed_code_sizes , packed_pos) ; let counts = & mut h . count [HUFF_CODES_TABLE] ; if self . repeat_count != 0 { if self . repeat_count < 3 { counts [self . prev_code_size as usize] = counts [self . prev_code_size as usize] . wrapping_add (self . repeat_count) ; let code = self . prev_code_size ; write (& [code , code , code] [.. self . repeat_count as usize]) ? ; } else { counts [16] = counts [16] . wrapping_add (1) ; write (& [16 , (self . repeat_count - 3) as u8] [..]) ? ; } self . repeat_count = 0 ; } Ok (()) } fn zero_code_size (& mut self , packed_code_sizes : & mut [u8] , packed_pos : & mut usize , h : & mut HuffmanOxide ,) -> Result < () > { let mut write = | buf | write (buf , packed_code_sizes , packed_pos) ; let counts = & mut h . count [HUFF_CODES_TABLE] ; if self . z_count != 0 { if self . z_count < 3 { counts [0] = counts [0] . wrapping_add (self . z_count as u16) ; write (& [0 , 0 , 0] [.. self . z_count as usize]) ? ; } else if self . z_count <= 10 { counts [17] = counts [17] . wrapping_add (1) ; write (& [17 , (self . z_count - 3) as u8] [..]) ? ; } else { counts [18] = counts [18] . wrapping_add (1) ; write (& [18 , (self . z_count - 11) as u8] [..]) ? ; } self . z_count = 0 ; } Ok (()) } }
    };
}

impl_70!()