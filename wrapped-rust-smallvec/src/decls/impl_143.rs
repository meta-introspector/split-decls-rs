macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < T , const N : usize > Eq for SmallVec < T , N > where T : Eq { }
    };
}

impl_143!();