macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl From < TimeSpec > for Duration { fn from (timespec : TimeSpec) -> Self { Duration :: new (timespec . 0 . tv_sec as u64 , timespec . 0 . tv_nsec as u32) } }
    };
}

impl_160!()