macro_rules! deps {
    () => {
        MatchKind!();
        MatchError!();
        MatchErrorKind!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl MatchError { # [doc = " Create a new error value with the given kind."] # [doc = ""] # [doc = " This is a more verbose version of the kind-specific constructors, e.g.,"] # [doc = " `MatchError::unsupported_stream`."] pub fn new (kind : MatchErrorKind) -> MatchError { MatchError (alloc :: boxed :: Box :: new (kind)) } # [doc = " Returns a reference to the underlying error kind."] pub fn kind (& self) -> & MatchErrorKind { & self . 0 } # [doc = " Create a new \"invalid anchored search\" error. This occurs when the"] # [doc = " caller requests an anchored search but where anchored searches aren't"] # [doc = " supported."] # [doc = ""] # [doc = " This is the same as calling `MatchError::new` with a"] # [doc = " [`MatchErrorKind::InvalidInputAnchored`] kind."] pub fn invalid_input_anchored () -> MatchError { MatchError :: new (MatchErrorKind :: InvalidInputAnchored) } # [doc = " Create a new \"invalid unanchored search\" error. This occurs when the"] # [doc = " caller requests an unanchored search but where unanchored searches"] # [doc = " aren't supported."] # [doc = ""] # [doc = " This is the same as calling `MatchError::new` with a"] # [doc = " [`MatchErrorKind::InvalidInputUnanchored`] kind."] pub fn invalid_input_unanchored () -> MatchError { MatchError :: new (MatchErrorKind :: InvalidInputUnanchored) } # [doc = " Create a new \"unsupported stream search\" error. This occurs when the"] # [doc = " caller requests a stream search while using an Aho-Corasick automaton"] # [doc = " with a match kind other than [`MatchKind::Standard`]."] # [doc = ""] # [doc = " The match kind given should be the match kind of the automaton. It"] # [doc = " should never be `MatchKind::Standard`."] pub fn unsupported_stream (got : MatchKind) -> MatchError { MatchError :: new (MatchErrorKind :: UnsupportedStream { got }) } # [doc = " Create a new \"unsupported overlapping search\" error. This occurs when"] # [doc = " the caller requests an overlapping search while using an Aho-Corasick"] # [doc = " automaton with a match kind other than [`MatchKind::Standard`]."] # [doc = ""] # [doc = " The match kind given should be the match kind of the automaton. It"] # [doc = " should never be `MatchKind::Standard`."] pub fn unsupported_overlapping (got : MatchKind) -> MatchError { MatchError :: new (MatchErrorKind :: UnsupportedOverlapping { got }) } # [doc = " Create a new \"unsupported empty pattern\" error. This occurs when the"] # [doc = " caller requests a search for which matching an automaton that contains"] # [doc = " an empty pattern string is not supported."] pub fn unsupported_empty () -> MatchError { MatchError :: new (MatchErrorKind :: UnsupportedEmpty) } }
    };
}

impl_344!();