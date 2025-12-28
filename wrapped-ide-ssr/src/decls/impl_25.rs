macro_rules! deps {
    () => {
        PlaceholderMatch!();
        SsrMatches!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl PlaceholderMatch { fn from_range (range : FileRange) -> Self { Self { range , inner_matches : SsrMatches :: default () , autoderef_count : 0 , autoref_kind : ast :: SelfParamKind :: Owned , } } }
    };
}

impl_25!()