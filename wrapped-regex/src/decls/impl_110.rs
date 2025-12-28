macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < 'r , 'h > core :: iter :: FusedIterator for Split < 'r , 'h > { }
    };
}

impl_110!()