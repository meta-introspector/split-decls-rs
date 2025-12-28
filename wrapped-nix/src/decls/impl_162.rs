macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl AsMut < timespec > for TimeSpec { fn as_mut (& mut self) -> & mut timespec { & mut self . 0 } }
    };
}

impl_162!();