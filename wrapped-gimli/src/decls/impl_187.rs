macro_rules! deps {
    () => {
        Reader!();
        UnwindSection!();
        DebugFrame!();
        DebugFrameOffset!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < R : Reader > UnwindSection < R > for DebugFrame < R > { type Offset = DebugFrameOffset < R :: Offset > ; }
    };
}

impl_187!()