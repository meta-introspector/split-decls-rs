macro_rules! deps {
    () => {
        Utf8Chunks!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < 'a > :: core :: iter :: FusedIterator for Utf8Chunks < 'a > { }
    };
}

impl_234!()