macro_rules! deps {
    () => {
        HexError!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for HexError { }
    };
}

impl_49!()