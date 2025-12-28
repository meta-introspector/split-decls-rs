macro_rules! deps {
    () => {
        Binding!();
        Time!();
    };
}

macro_rules! impl_790 {
    () => {
        deps!();
        impl Binding for Time { type Raw = raw :: git_time ; unsafe fn from_raw (raw : raw :: git_time) -> Time { Time { raw } } fn raw (& self) -> raw :: git_time { self . raw } }
    };
}

impl_790!()