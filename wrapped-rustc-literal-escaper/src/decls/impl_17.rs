macro_rules! deps {
    () => {
        CheckRaw!();
        EscapeError!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl CheckRaw for [u8] { type RawUnit = u8 ; # [inline] fn char2raw_unit (c : char) -> Result < Self :: RawUnit , EscapeError > { char2byte (c) } }
    };
}

impl_17!()