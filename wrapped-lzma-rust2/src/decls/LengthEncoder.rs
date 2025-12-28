macro_rules! deps {
    () => {
        LengthCoder!();
    };
}

macro_rules! LengthEncoder {
    () => {
        deps!();
        pub (crate) struct LengthEncoder { coder : LengthCoder , counters : Vec < i32 > , prices : Vec < Vec < u32 > > , }
    };
}

LengthEncoder!();