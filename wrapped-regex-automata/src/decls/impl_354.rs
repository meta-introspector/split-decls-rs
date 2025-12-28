macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl < 'r , 'h > core :: iter :: FusedIterator for Split < 'r , 'h > { }
    };
}

impl_354!();