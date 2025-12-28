macro_rules! deps {
    () => {
        OneIter!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < 'a , 'h > core :: iter :: FusedIterator for OneIter < 'a , 'h > { }
    };
}

impl_246!()