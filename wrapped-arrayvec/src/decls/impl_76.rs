macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T , const CAP : usize > Eq for ArrayVec < T , CAP > where T : Eq { }
    };
}

impl_76!();