macro_rules! deps {
    () => {
        TupleWindows!();
        HomogeneousTuple!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        impl < I , T > ExactSizeIterator for TupleWindows < I , T > where I : ExactSizeIterator < Item = T :: Item > , T : HomogeneousTuple + Clone , T :: Item : Clone , { }
    };
}

impl_517!()