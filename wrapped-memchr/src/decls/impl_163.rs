macro_rules! deps {
    () => {
        ThreeIter!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < 'a , 'h > core :: iter :: FusedIterator for ThreeIter < 'a , 'h > { }
    };
}

impl_163!();