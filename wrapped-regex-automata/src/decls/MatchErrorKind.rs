macro_rules! deps {
    () => {
        Anchored!();
        MatchError!();
        DFA!();
    };
}

macro_rules! MatchErrorKind {
    () => {
        deps!();
        # [doc = " The underlying kind of a [`MatchError`]."] # [doc = ""] # [doc = " This is a **non-exhaustive** enum. That means new variants may be added in"] # [doc = " a semver-compatible release."] # [non_exhaustive] # [derive (Clone , Debug , Eq , PartialEq)] pub enum MatchErrorKind { # [doc = " The search saw a \"quit\" byte at which it was instructed to stop"] # [doc = " searching."] Quit { # [doc = " The \"quit\" byte that was observed that caused the search to stop."] byte : u8 , # [doc = " The offset at which the quit byte was observed."] offset : usize , } , # [doc = " The search, based on heuristics, determined that it would be better"] # [doc = " to stop, typically to provide the caller an opportunity to use an"] # [doc = " alternative regex engine."] # [doc = ""] # [doc = " Currently, the only way for this to occur is via the lazy DFA and"] # [doc = " only when it is configured to do so (it will not return this error by"] # [doc = " default)."] GaveUp { # [doc = " The offset at which the search stopped. This corresponds to the"] # [doc = " position immediately following the last byte scanned."] offset : usize , } , # [doc = " This error occurs if the haystack given to the regex engine was too"] # [doc = " long to be searched. This occurs, for example, with regex engines"] # [doc = " like the bounded backtracker that have a configurable fixed amount of"] # [doc = " capacity that is tied to the length of the haystack. Anything beyond"] # [doc = " that configured limit will result in an error at search time."] HaystackTooLong { # [doc = " The length of the haystack that exceeded the limit."] len : usize , } , # [doc = " An error indicating that a particular type of anchored search was"] # [doc = " requested, but that the regex engine does not support it."] # [doc = ""] # [doc = " Note that this error should not be returned by a regex engine simply"] # [doc = " because the pattern ID is invalid (i.e., equal to or exceeds the number"] # [doc = " of patterns in the regex). In that case, the regex engine should report"] # [doc = " a non-match."] UnsupportedAnchored { # [doc = " The anchored mode given that is unsupported."] mode : Anchored , } , }
    };
}

MatchErrorKind!();