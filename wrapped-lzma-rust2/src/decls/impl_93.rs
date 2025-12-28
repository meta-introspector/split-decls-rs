macro_rules! deps {
    () => {
        RangeDecoder!();
        RangeDecoderBuffer!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl RangeDecoder < RangeDecoderBuffer > { pub (crate) fn new_buffer (size : usize) -> Self { Self { inner : RangeDecoderBuffer :: new (size - 5) , code : 0 , range : 0 , } } }
    };
}

impl_93!()