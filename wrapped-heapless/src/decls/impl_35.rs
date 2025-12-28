macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T > FusedIterator for IterMut < '_ , T > { }
    };
}

impl_35!()