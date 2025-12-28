macro_rules! deps {
    () => {
        NoAnn!();
        PpAnn!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl PpAnn for NoAnn { }
    };
}

impl_33!();