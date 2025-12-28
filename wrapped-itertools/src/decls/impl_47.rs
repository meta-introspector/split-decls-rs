macro_rules! deps {
    () => {
        MapSpecialCase!();
        MapSpecialCaseFn!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < I , R > ExactSizeIterator for MapSpecialCase < I , R > where I : ExactSizeIterator , R : MapSpecialCaseFn < I :: Item > , { }
    };
}

impl_47!()