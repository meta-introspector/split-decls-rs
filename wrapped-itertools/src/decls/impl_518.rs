macro_rules! deps {
    () => {
        HomogeneousTuple!();
        TupleWindows!();
    };
}

macro_rules! impl_518 {
    () => {
        deps!();
        impl < I , T > FusedIterator for TupleWindows < I , T > where I : FusedIterator < Item = T :: Item > , T : HomogeneousTuple + Clone , T :: Item : Clone , { }
    };
}

impl_518!()