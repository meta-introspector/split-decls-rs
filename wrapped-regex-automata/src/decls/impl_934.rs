macro_rules! deps {
    () => {
        MatchError!();
        Anchored!();
        MatchErrorKind!();
    };
}

macro_rules! impl_934 {
    () => {
        deps!();
        impl MatchError { # [doc = " Create a new error value with the given kind."] # [doc = ""] # [doc = " This is a more verbose version of the kind-specific constructors,"] # [doc = " e.g., `MatchError::quit`."] pub fn new (kind : MatchErrorKind) -> MatchError { # [cfg (feature = "alloc")] { MatchError (alloc :: boxed :: Box :: new (kind)) } # [cfg (not (feature = "alloc"))] { MatchError (kind) } } # [doc = " Returns a reference to the underlying error kind."] pub fn kind (& self) -> & MatchErrorKind { & self . 0 } # [doc = " Create a new \"quit\" error. The given `byte` corresponds to the value"] # [doc = " that tripped a search's quit condition, and `offset` corresponds to the"] # [doc = " location in the haystack at which the search quit."] # [doc = ""] # [doc = " This is the same as calling `MatchError::new` with a"] # [doc = " [`MatchErrorKind::Quit`] kind."] pub fn quit (byte : u8 , offset : usize) -> MatchError { MatchError :: new (MatchErrorKind :: Quit { byte , offset }) } # [doc = " Create a new \"gave up\" error. The given `offset` corresponds to the"] # [doc = " location in the haystack at which the search gave up."] # [doc = ""] # [doc = " This is the same as calling `MatchError::new` with a"] # [doc = " [`MatchErrorKind::GaveUp`] kind."] pub fn gave_up (offset : usize) -> MatchError { MatchError :: new (MatchErrorKind :: GaveUp { offset }) } # [doc = " Create a new \"haystack too long\" error. The given `len` corresponds to"] # [doc = " the length of the haystack that was problematic."] # [doc = ""] # [doc = " This is the same as calling `MatchError::new` with a"] # [doc = " [`MatchErrorKind::HaystackTooLong`] kind."] pub fn haystack_too_long (len : usize) -> MatchError { MatchError :: new (MatchErrorKind :: HaystackTooLong { len }) } # [doc = " Create a new \"unsupported anchored\" error. This occurs when the caller"] # [doc = " requests a search with an anchor mode that is not supported by the"] # [doc = " regex engine."] # [doc = ""] # [doc = " This is the same as calling `MatchError::new` with a"] # [doc = " [`MatchErrorKind::UnsupportedAnchored`] kind."] pub fn unsupported_anchored (mode : Anchored) -> MatchError { MatchError :: new (MatchErrorKind :: UnsupportedAnchored { mode }) } }
    };
}

impl_934!();