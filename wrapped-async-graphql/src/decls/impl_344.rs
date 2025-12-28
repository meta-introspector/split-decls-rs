macro_rules! deps {
    () => {
        Parents!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl std :: iter :: FusedIterator for Parents < '_ > { }
    };
}

impl_344!()