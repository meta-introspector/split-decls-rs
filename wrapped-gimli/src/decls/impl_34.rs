macro_rules! deps {
    () => {
        EhFrameOffset!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T > From < T > for EhFrameOffset < T > { # [inline] fn from (o : T) -> Self { EhFrameOffset (o) } }
    };
}

impl_34!()