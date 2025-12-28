macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for ParseError { }
    };
}

impl_126!()