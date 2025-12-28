macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        unsafe impl < 'a , T : Send , const CAP : usize > Send for Drain < 'a , T , CAP > { }
    };
}

impl_60!();