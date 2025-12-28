macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_904 {
    () => {
        deps!();
        impl < St : Stream + Unpin > ExactSizeIterator for Iter < '_ , St > { }
    };
}

impl_904!()