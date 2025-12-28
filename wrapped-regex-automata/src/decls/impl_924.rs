macro_rules! deps {
    () => {
        PatternSetInsertError!();
    };
}

macro_rules! impl_924 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for PatternSetInsertError { }
    };
}

impl_924!();