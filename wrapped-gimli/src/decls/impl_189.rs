macro_rules! deps {
    () => {
        Reader!();
        UnwindSection!();
        EhFrame!();
        EhFrameOffset!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < R : Reader > UnwindSection < R > for EhFrame < R > { type Offset = EhFrameOffset < R :: Offset > ; }
    };
}

impl_189!();