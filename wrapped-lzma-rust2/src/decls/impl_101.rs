macro_rules! deps {
    () => {
        RangeDecoderBuffer!();
        Result!();
        RangeReader!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl RangeReader for RangeDecoderBuffer { # [inline (always)] fn read_u8 (& mut self) -> u8 { let byte = * self . buf . get (self . pos) . unwrap_or (& 1) ; self . pos += 1 ; byte } fn try_read_u8 (& mut self) -> crate :: Result < u8 > { self . buf . get (self . pos) . copied () . ok_or_else (error_eof) } # [inline (always)] fn read_u32_be (& mut self) -> crate :: Result < u32 > { let b = u32 :: from_be_bytes (self . buf [self . pos .. self . pos + 4] . try_into () . unwrap ()) ; self . pos += 4 ; Ok (b) } # [inline (always)] fn is_buffer (& self) -> bool { true } # [inline (always)] fn pos (& self) -> usize { self . pos } # [inline (always)] fn set_pos (& mut self , pos : usize) { self . pos = pos ; } # [inline (always)] fn buf (& self) -> & [u8] { self . buf . as_slice () } }
    };
}

impl_101!()