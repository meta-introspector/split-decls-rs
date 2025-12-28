macro_rules! deps {
    () => {
        BoundedBacktracker!();
    };
}

macro_rules! BoundedBacktrackerEngine {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct BoundedBacktrackerEngine (# [cfg (feature = "nfa-backtrack")] backtrack :: BoundedBacktracker , # [cfg (not (feature = "nfa-backtrack"))] () ,) ;
    };
}

BoundedBacktrackerEngine!();