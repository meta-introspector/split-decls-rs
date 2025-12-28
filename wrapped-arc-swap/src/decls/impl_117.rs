macro_rules! deps {
    () => {
        RefCnt!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < T : RefCnt > Protected < T > for T { # [inline] fn from_inner (ptr : T) -> Self { ptr } # [inline] fn into_inner (self) -> T { self } }
    };
}

impl_117!();