macro_rules! deps {
    () => {
        SliceRead!();
    };
}

macro_rules! impl_580 {
    () => {
        deps!();
        impl < 'a > private :: Sealed for SliceRead < 'a > { }
    };
}

impl_580!();