macro_rules! deps {
    () => {
        Union!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < 'a > FusedIterator for Union < 'a > { }
    };
}

impl_106!()