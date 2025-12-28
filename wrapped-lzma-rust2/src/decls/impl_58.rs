macro_rules! deps {
    () => {
        Result!();
        ByteReader!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < T : Read > ByteReader for T { # [inline (always)] fn read_u8 (& mut self) -> Result < u8 > { let mut buf = [0 ; 1] ; self . read_exact (& mut buf) ? ; Ok (buf [0]) } # [inline (always)] fn read_u16 (& mut self) -> Result < u16 > { let mut buf = [0 ; 2] ; self . read_exact (buf . as_mut ()) ? ; Ok (u16 :: from_le_bytes (buf)) } # [inline (always)] fn read_u16_be (& mut self) -> Result < u16 > { let mut buf = [0 ; 2] ; self . read_exact (buf . as_mut ()) ? ; Ok (u16 :: from_be_bytes (buf)) } # [inline (always)] fn read_u32 (& mut self) -> Result < u32 > { let mut buf = [0 ; 4] ; self . read_exact (buf . as_mut ()) ? ; Ok (u32 :: from_le_bytes (buf)) } # [inline (always)] fn read_u32_be (& mut self) -> Result < u32 > { let mut buf = [0 ; 4] ; self . read_exact (buf . as_mut ()) ? ; Ok (u32 :: from_be_bytes (buf)) } # [inline (always)] fn read_u64 (& mut self) -> Result < u64 > { let mut buf = [0 ; 8] ; self . read_exact (buf . as_mut ()) ? ; Ok (u64 :: from_le_bytes (buf)) } }
    };
}

impl_58!()