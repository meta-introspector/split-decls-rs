macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        unsafe impl < T : Send , const N : usize > Send for SmallVec < T , N > { }
    };
}

impl_83!();