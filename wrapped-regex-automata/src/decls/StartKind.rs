macro_rules! deps {
    () => {
        Anchored!();
        DFA!();
    };
}

macro_rules! StartKind {
    () => {
        deps!();
        # [doc = " The kind of anchored starting configurations to support in a DFA."] # [doc = ""] # [doc = " Fully compiled DFAs need to be explicitly configured as to which anchored"] # [doc = " starting configurations to support. The reason for not just supporting"] # [doc = " everything unconditionally is that it can use more resources (such as"] # [doc = " memory and build time). The downside of this is that if you try to execute"] # [doc = " a search using an [`Anchored`](crate::Anchored) mode that is not supported"] # [doc = " by the DFA, then the search will return an error."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum StartKind { # [doc = " Support both anchored and unanchored searches."] Both , # [doc = " Support only unanchored searches. Requesting an anchored search will"] # [doc = " panic."] # [doc = ""] # [doc = " Note that even if an unanchored search is requested, the pattern itself"] # [doc = " may still be anchored. For example, `^abc` will only match `abc` at the"] # [doc = " start of a haystack. This will remain true, even if the regex engine"] # [doc = " only supported unanchored searches."] Unanchored , # [doc = " Support only anchored searches. Requesting an unanchored search will"] # [doc = " panic."] Anchored , }
    };
}

StartKind!()