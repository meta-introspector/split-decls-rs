macro_rules! deps {
    () => {
        NodeTrait!();
    };
}

macro_rules! impl_879 {
    () => {
        deps!();
        impl < N > NodeTrait for N where N : Copy + Ord + Hash { }
    };
}

impl_879!()