macro_rules! deps {
    () => {
        CsrError!();
    };
}

macro_rules! impl_513 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for CsrError { }
    };
}

impl_513!();