macro_rules! deps {
    () => {
        PathspecMatch!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl From < gix_pathspec :: search :: MatchKind > for PathspecMatch { fn from (kind : gix_pathspec :: search :: MatchKind) -> Self { match kind { gix_pathspec :: search :: MatchKind :: Always => Self :: Always , gix_pathspec :: search :: MatchKind :: Prefix => Self :: Prefix , gix_pathspec :: search :: MatchKind :: WildcardMatch => Self :: WildcardMatch , gix_pathspec :: search :: MatchKind :: Verbatim => Self :: Verbatim , } } }
    };
}

impl_7!()