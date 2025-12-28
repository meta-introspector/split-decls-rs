macro_rules! deps {
    () => {
        Memchr!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl < 'h > core :: iter :: FusedIterator for Memchr < 'h > { }
    };
}

impl_306!()