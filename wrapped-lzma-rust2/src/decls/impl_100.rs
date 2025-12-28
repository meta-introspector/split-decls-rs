macro_rules! deps {
    () => {
        Result!();
        RangeReader!();
        Read!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < T : Read > RangeReader for T { # [inline (always)] fn read_u8 (& mut self) -> u8 { let mut buf = [0 ; 1] ; match self . read_exact (& mut buf) { Ok (_) => buf [0] , Err (_) => 1 , } } fn try_read_u8 (& mut self) -> crate :: Result < u8 > { let mut buf = [0 ; 1] ; self . read_exact (& mut buf) ? ; Ok (buf [0]) } # [inline (always)] fn read_u32_be (& mut self) -> crate :: Result < u32 > { let mut buf = [0 ; 4] ; self . read_exact (buf . as_mut ()) ? ; Ok (u32 :: from_be_bytes (buf)) } }
    };
}

impl_100!()