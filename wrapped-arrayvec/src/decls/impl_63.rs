macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < 'a , T : 'a , const CAP : usize > ExactSizeIterator for Drain < 'a , T , CAP > { }
    };
}

impl_63!()