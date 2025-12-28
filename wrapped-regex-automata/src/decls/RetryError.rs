macro_rules! deps {
    () => {
        RetryQuadraticError!();
        PikeVM!();
        RetryFailError!();
        DFA!();
    };
}

macro_rules! RetryError {
    () => {
        deps!();
        # [doc = " An error that occurs when a search should be retried."] # [doc = ""] # [doc = " This retry error distinguishes between two different failure modes."] # [doc = ""] # [doc = " The first is one where potential quadratic behavior has been detected."] # [doc = " In this case, whatever optimization that led to this behavior should be"] # [doc = " stopped, and the next best strategy should be used."] # [doc = ""] # [doc = " The second indicates that the underlying regex engine has failed for some"] # [doc = " reason. This usually occurs because either a lazy DFA's cache has become"] # [doc = " ineffective or because a non-ASCII byte has been seen *and* a Unicode word"] # [doc = " boundary was used in one of the patterns. In this failure case, a different"] # [doc = " regex engine that won't fail in these ways (PikeVM, backtracker or the"] # [doc = " one-pass DFA) should be used."] # [doc = ""] # [doc = " This is an internal error only and should never bleed into the public"] # [doc = " API."] # [derive (Debug)] pub (crate) enum RetryError { Quadratic (RetryQuadraticError) , Fail (RetryFailError) , }
    };
}

RetryError!()