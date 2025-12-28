macro_rules! deps {
    () => {
        LayoutError!();
    };
}

macro_rules! impl_658 {
    () => {
        deps!();
        impl std :: error :: Error for LayoutError { }
    };
}

impl_658!()