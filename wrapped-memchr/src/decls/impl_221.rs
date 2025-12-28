macro_rules! deps {
    () => {
        OneIter!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < 'a , 'h > core :: iter :: FusedIterator for OneIter < 'a , 'h > { }
    };
}

impl_221!()