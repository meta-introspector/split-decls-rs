macro_rules! deps {
    () => {
        UnwindOffset!();
        DebugFrameOffset!();
        ReaderOffset!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < T > UnwindOffset < T > for DebugFrameOffset < T > where T : ReaderOffset , { # [inline] fn into (self) -> T { self . 0 } }
    };
}

impl_182!();