macro_rules! deps {
    () => {
        Intersection!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a > FusedIterator for Intersection < 'a > { }
    };
}

impl_33!()