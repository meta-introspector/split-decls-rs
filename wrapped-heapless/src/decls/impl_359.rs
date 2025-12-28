macro_rules! deps {
    () => {
        Kind!();
        Min!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl Kind for Min { fn ordering () -> Ordering { Ordering :: Less } }
    };
}

impl_359!()