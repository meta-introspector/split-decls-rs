macro_rules! deps {
    () => {
        Deque!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T : Eq , const N : usize > Eq for Deque < T , N > { }
    };
}

impl_51!()