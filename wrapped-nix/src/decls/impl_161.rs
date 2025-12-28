macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl AsRef < timespec > for TimeSpec { fn as_ref (& self) -> & timespec { & self . 0 } }
    };
}

impl_161!();