macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for ParseError { }
    };
}

impl_26!();