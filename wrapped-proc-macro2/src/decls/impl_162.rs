macro_rules! deps {
    () => {
        NonZeroChar!();
        EscapeError!();
        CheckRaw!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl CheckRaw for CStr { type RawUnit = NonZeroChar ; # [inline] fn char2raw_unit (c : char) -> Result < Self :: RawUnit , EscapeError > { NonZeroChar :: new (c) . ok_or (EscapeError :: NulInCStr) } }
    };
}

impl_162!();