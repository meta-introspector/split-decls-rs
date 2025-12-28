macro_rules! deps {
    () => {
        PollFn!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < F > fmt :: Debug for PollFn < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("PollFn") . finish () } }
    };
}

impl_61!();