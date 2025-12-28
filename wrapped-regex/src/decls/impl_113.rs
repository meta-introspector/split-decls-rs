macro_rules! deps {
    () => {
        SplitN!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < 'r , 'h > core :: iter :: FusedIterator for SplitN < 'r , 'h > { }
    };
}

impl_113!();