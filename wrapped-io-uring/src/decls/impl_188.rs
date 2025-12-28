macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl From < std :: time :: Duration > for Timespec { fn from (value : std :: time :: Duration) -> Self { Timespec :: new () . sec (value . as_secs ()) . nsec (value . subsec_nanos ()) } }
    };
}

impl_188!();