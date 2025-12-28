macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! MaxRecursionReached {
    () => {
        deps!();
        # [doc = " Error indicating that the maximum recursion depth has been reached while calculating [`Arbitrary::size_hint`]()"] # [derive (Debug , Clone)] # [non_exhaustive] pub struct MaxRecursionReached { }
    };
}

MaxRecursionReached!()