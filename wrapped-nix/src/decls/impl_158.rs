macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl From < timespec > for TimeSpec { fn from (ts : timespec) -> Self { Self (ts) } }
    };
}

impl_158!()