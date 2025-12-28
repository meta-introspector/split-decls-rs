macro_rules! deps {
    () => {
        Union!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < 'a > FusedIterator for Union < 'a > { }
    };
}

impl_37!()