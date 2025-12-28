macro_rules! deps {
    () => {
        Min!();
        Kind!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl Kind for Min { fn ordering () -> Ordering { Ordering :: Less } }
    };
}

impl_423!()