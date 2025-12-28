macro_rules! deps {
    () => {
        EscapeError!();
        CheckRaw!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl CheckRaw for str { type RawUnit = char ; # [inline] fn char2raw_unit (c : char) -> Result < Self :: RawUnit , EscapeError > { Ok (c) } }
    };
}

impl_7!()