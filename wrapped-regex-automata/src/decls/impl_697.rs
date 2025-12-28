macro_rules! deps {
    () => {
        UnicodeWordBoundaryError!();
    };
}

macro_rules! impl_697 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for UnicodeWordBoundaryError { }
    };
}

impl_697!();