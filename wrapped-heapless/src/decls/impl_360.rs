macro_rules! deps {
    () => {
        Kind!();
        Max!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl Kind for Max { fn ordering () -> Ordering { Ordering :: Greater } }
    };
}

impl_360!()