macro_rules! deps {
    () => {
        MatchError!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for MatchError { }
    };
}

impl_346!()