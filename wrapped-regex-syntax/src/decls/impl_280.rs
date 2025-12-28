macro_rules! deps {
    () => {
        CaseFoldError!();
        Error!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for CaseFoldError { }
    };
}

impl_280!()