macro_rules! deps {
    () => {
        CheckRaw!();
        EscapeError!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl CheckRaw for CStr { type RawUnit = NonZero < char > ; # [inline] fn char2raw_unit (c : char) -> Result < Self :: RawUnit , EscapeError > { NonZero :: new (c) . ok_or (EscapeError :: NulInCStr) } }
    };
}

impl_10!()