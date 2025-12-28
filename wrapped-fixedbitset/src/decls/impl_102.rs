macro_rules! deps {
    () => {
        Intersection!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'a > FusedIterator for Intersection < 'a > { }
    };
}

impl_102!()