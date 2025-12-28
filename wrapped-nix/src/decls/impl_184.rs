macro_rules! deps {
    () => {
        TimeVal!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl ops :: Neg for TimeVal { type Output = TimeVal ; fn neg (self) -> TimeVal { TimeVal :: microseconds (- self . num_microseconds ()) } }
    };
}

impl_184!()