macro_rules! deps {
    () => {
        TimeVal!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl From < timeval > for TimeVal { fn from (tv : timeval) -> Self { TimeVal (tv) } }
    };
}

impl_190!();