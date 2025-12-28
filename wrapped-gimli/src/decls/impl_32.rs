macro_rules! deps {
    () => {
        DebugFrameOffset!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T > From < T > for DebugFrameOffset < T > { # [inline] fn from (o : T) -> Self { DebugFrameOffset (o) } }
    };
}

impl_32!();