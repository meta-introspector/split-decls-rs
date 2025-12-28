macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! BoundedBacktrackerCache {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct BoundedBacktrackerCache (# [cfg (feature = "nfa-backtrack")] Option < backtrack :: Cache > , # [cfg (not (feature = "nfa-backtrack"))] () ,) ;
    };
}

BoundedBacktrackerCache!();