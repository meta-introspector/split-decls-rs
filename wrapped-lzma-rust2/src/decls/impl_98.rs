macro_rules! deps {
    () => {
        RangeDecoderBuffer!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl RangeDecoderBuffer { pub (crate) fn new (len : usize) -> Self { Self { buf : vec ! [0 ; len] , pos : len , } } }
    };
}

impl_98!();