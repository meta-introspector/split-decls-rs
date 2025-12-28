macro_rules! deps {
    () => {
        DFA!();
        MatchError!();
    };
}

macro_rules! RetryFailError {
    () => {
        deps!();
        # [doc = " An error that occurs when a regex engine \"gives up\" for some reason before"] # [doc = " finishing a search. Usually this occurs because of heuristic Unicode word"] # [doc = " boundary support or because of ineffective cache usage in the lazy DFA."] # [doc = ""] # [doc = " When this error occurs, callers should retry the regex search with a"] # [doc = " different regex engine."] # [doc = ""] # [doc = " Note that this has convenient `From` impls that will automatically"] # [doc = " convert a `MatchError` into this error. This works because the meta"] # [doc = " regex engine internals guarantee that errors like `HaystackTooLong` and"] # [doc = " `UnsupportedAnchored` will never occur. The only errors left are `Quit` and"] # [doc = " `GaveUp`, which both correspond to this \"failure\" error."] # [derive (Debug)] pub (crate) struct RetryFailError { offset : usize , }
    };
}

RetryFailError!()