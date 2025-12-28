macro_rules! deps {
    () => {
        Compat01As03!();
    };
}

macro_rules! impl_1006 {
    () => {
        deps!();
        impl < T > Unpin for Compat01As03 < T > { }
    };
}

impl_1006!();