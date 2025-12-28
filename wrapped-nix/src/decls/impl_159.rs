macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl From < Duration > for TimeSpec { fn from (duration : Duration) -> Self { Self :: from_duration (duration) } }
    };
}

impl_159!();