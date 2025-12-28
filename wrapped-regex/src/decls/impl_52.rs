macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < 'r , 'h > core :: iter :: FusedIterator for Split < 'r , 'h > { }
    };
}

impl_52!()