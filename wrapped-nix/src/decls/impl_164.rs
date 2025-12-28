macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl PartialOrd for TimeSpec { fn partial_cmp (& self , other : & TimeSpec) -> Option < cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_164!()