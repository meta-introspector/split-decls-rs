macro_rules! deps {
    () => {
        Safety!();
        HeaderSafety!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl From < Safety > for HeaderSafety { fn from (v : Safety) -> Self { Self :: Normal (v) } }
    };
}

impl_319!();