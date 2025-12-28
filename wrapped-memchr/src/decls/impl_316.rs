macro_rules! deps {
    () => {
        Memchr3!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl < 'h > core :: iter :: FusedIterator for Memchr3 < 'h > { }
    };
}

impl_316!();