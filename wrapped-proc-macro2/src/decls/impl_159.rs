macro_rules! deps {
    () => {
        CheckRaw!();
        EscapeError!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl CheckRaw for str { type RawUnit = char ; # [inline] fn char2raw_unit (c : char) -> Result < Self :: RawUnit , EscapeError > { Ok (c) } }
    };
}

impl_159!();