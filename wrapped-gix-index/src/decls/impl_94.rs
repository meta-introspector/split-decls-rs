macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl From < Time > for SystemTime { fn from (s : Time) -> Self { std :: time :: UNIX_EPOCH + std :: time :: Duration :: new (s . secs . into () , s . nsecs) } }
    };
}

impl_94!();