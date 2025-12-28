macro_rules! deps {
    () => {
        TimeVal!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl PartialOrd for TimeVal { fn partial_cmp (& self , other : & TimeVal) -> Option < cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_181!()