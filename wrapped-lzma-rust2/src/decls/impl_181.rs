macro_rules! deps {
    () => {
        LzmaEncoderTrait!();
        LzmaEncoder!();
        LzmaEncoderModes!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl LzmaEncoderTrait for LzmaEncoderModes { fn get_next_symbol (& mut self , encoder : & mut LzmaEncoder) -> u32 { match self { LzmaEncoderModes :: Fast (a) => a . get_next_symbol (encoder) , LzmaEncoderModes :: Normal (a) => a . get_next_symbol (encoder) , } } fn reset (& mut self) { match self { LzmaEncoderModes :: Fast (a) => a . reset () , LzmaEncoderModes :: Normal (a) => a . reset () , } } }
    };
}

impl_181!()