macro_rules! RangeEncoderBuffer {
    () => {
        pub (crate) struct RangeEncoderBuffer { buf : Vec < u8 > , pos : usize , }
    };
}

RangeEncoderBuffer!();