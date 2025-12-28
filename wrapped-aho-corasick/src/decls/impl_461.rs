macro_rules! deps {
    () => {
        MatchKind!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl MatchKind { # [inline] pub (crate) fn is_standard (& self) -> bool { matches ! (* self , MatchKind :: Standard) } # [inline] pub (crate) fn is_leftmost (& self) -> bool { matches ! (* self , MatchKind :: LeftmostFirst | MatchKind :: LeftmostLongest) } # [inline] pub (crate) fn is_leftmost_first (& self) -> bool { matches ! (* self , MatchKind :: LeftmostFirst) } # [doc = " Convert this match kind into a packed match kind. If this match kind"] # [doc = " corresponds to standard semantics, then this returns None, since"] # [doc = " packed searching does not support standard semantics."] # [inline] pub (crate) fn as_packed (& self) -> Option < crate :: packed :: MatchKind > { match * self { MatchKind :: Standard => None , MatchKind :: LeftmostFirst => { Some (crate :: packed :: MatchKind :: LeftmostFirst) } MatchKind :: LeftmostLongest => { Some (crate :: packed :: MatchKind :: LeftmostLongest) } } } }
    };
}

impl_461!();