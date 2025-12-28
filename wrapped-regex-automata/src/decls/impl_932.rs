macro_rules! deps {
    () => {
        MatchKind!();
    };
}

macro_rules! impl_932 {
    () => {
        deps!();
        impl MatchKind { # [cfg (feature = "alloc")] pub (crate) fn continue_past_first_match (& self) -> bool { * self == MatchKind :: All } }
    };
}

impl_932!();