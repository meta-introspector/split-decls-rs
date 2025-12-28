macro_rules! deps {
    () => {
        MatchKind!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl Default for MatchKind { fn default () -> MatchKind { MatchKind :: LeftmostFirst } }
    };
}

impl_104!();