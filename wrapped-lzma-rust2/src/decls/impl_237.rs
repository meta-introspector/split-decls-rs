macro_rules! deps {
    () => {
        Result!();
        RangeEncoderBuffer!();
        Write!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl RangeEncoderBuffer { pub (crate) fn new (size : usize) -> Self { Self { buf : vec ! [0 ; size] , pos : 0 , } } pub (crate) fn write_to < W : Write > (& self , out : & mut W) -> crate :: Result < () > { out . write_all (& self . buf [.. self . pos]) } }
    };
}

impl_237!();