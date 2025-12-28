macro_rules! deps {
    () => {
        Error!();
        MatchesError!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl std :: error :: Error for MatchesError { }
    };
}

impl_434!();