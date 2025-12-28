macro_rules! RangeDecoderBuffer {
    () => {
        pub (crate) struct RangeDecoderBuffer { buf : Vec < u8 > , pos : usize , }
    };
}

RangeDecoderBuffer!()