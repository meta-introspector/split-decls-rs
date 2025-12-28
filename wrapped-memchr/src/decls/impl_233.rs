macro_rules! deps {
    () => {
        ThreeIter!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < 'a , 'h > core :: iter :: FusedIterator for ThreeIter < 'a , 'h > { }
    };
}

impl_233!()