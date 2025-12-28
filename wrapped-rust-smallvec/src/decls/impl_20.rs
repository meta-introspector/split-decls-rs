macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T , const N : usize > core :: iter :: FusedIterator for Drain < '_ , T , N > { }
    };
}

impl_20!()