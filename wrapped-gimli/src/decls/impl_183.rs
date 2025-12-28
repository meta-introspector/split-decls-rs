macro_rules! deps {
    () => {
        EhFrameOffset!();
        ReaderOffset!();
        UnwindOffset!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < T > UnwindOffset < T > for EhFrameOffset < T > where T : ReaderOffset , { # [inline] fn into (self) -> T { self . 0 } }
    };
}

impl_183!();