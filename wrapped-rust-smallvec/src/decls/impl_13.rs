macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        unsafe impl < T : Send , const N : usize > Send for SmallVec < T , N > { }
    };
}

impl_13!()