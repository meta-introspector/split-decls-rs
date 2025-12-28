macro_rules! deps {
    () => {
        Kind!();
        Max!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl Kind for Max { fn ordering () -> Ordering { Ordering :: Greater } }
    };
}

impl_424!()