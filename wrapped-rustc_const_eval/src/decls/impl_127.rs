macro_rules! deps {
    () => {
        MayLeak!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl interpret :: MayLeak for ! { # [inline (always)] fn may_leak (self) -> bool { self } }
    };
}

impl_127!()