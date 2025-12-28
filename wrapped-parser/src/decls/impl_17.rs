macro_rules! deps {
    () => {
        FrontmatterError!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl std :: error :: Error for FrontmatterError { }
    };
}

impl_17!();