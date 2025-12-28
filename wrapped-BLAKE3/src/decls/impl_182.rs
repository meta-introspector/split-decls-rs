macro_rules! deps {
    () => {
        HexError!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for HexError { }
    };
}

impl_182!();