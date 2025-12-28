macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_906 {
    () => {
        deps!();
        impl < St : Stream + Unpin > ExactSizeIterator for IterMut < '_ , St > { }
    };
}

impl_906!()