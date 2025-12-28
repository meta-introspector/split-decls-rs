macro_rules! deps {
    () => {
        WalkTreePostfix!();
        ParallelIterator!();
    };
}

macro_rules! WalkTree {
    () => {
        deps!();
        # [doc = " ParallelIterator for arbitrary tree-shaped patterns."] # [doc = " Returned by the [`walk_tree()`] function."] # [derive (Debug)] pub struct WalkTree < S , B > (WalkTreePostfix < S , B >) ;
    };
}

WalkTree!();