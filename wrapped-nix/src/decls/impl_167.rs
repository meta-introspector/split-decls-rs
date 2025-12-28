macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl ops :: Neg for TimeSpec { type Output = TimeSpec ; fn neg (self) -> TimeSpec { TimeSpec :: nanoseconds (- self . num_nanoseconds ()) } }
    };
}

impl_167!()