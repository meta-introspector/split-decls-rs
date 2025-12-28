macro_rules! deps {
    () => {
        EncoderResult!();
    };
}

macro_rules! impl_515 {
    () => {
        deps!();
        impl EncoderResult { fn unmappable_from_bmp (bmp : u16) -> EncoderResult { EncoderResult :: Unmappable (:: core :: char :: from_u32 (u32 :: from (bmp)) . unwrap ()) } }
    };
}

impl_515!()