macro_rules! deps {
    () => {
        LzmaEncoder!();
    };
}

macro_rules! LzmaEncoderTrait {
    () => {
        deps!();
        pub (crate) trait LzmaEncoderTrait { fn get_next_symbol (& mut self , encoder : & mut LzmaEncoder) -> u32 ; fn reset (& mut self) { } }
    };
}

LzmaEncoderTrait!()