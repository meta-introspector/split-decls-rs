macro_rules! deps {
    () => {
        Memchr2!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl < 'h > core :: iter :: FusedIterator for Memchr2 < 'h > { }
    };
}

impl_311!()