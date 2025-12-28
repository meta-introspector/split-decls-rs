macro_rules! deps {
    () => {
        Tuples!();
        HomogeneousTuple!();
    };
}

macro_rules! impl_512 {
    () => {
        deps!();
        impl < I , T > ExactSizeIterator for Tuples < I , T > where I : ExactSizeIterator < Item = T :: Item > , T : HomogeneousTuple , { }
    };
}

impl_512!();