macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! WalkTreePrefix {
    () => {
        deps!();
        # [doc = " ParallelIterator for arbitrary tree-shaped patterns."] # [doc = " Returned by the [`walk_tree_prefix()`] function."] # [derive (Debug)] pub struct WalkTreePrefix < S , B > { initial_state : S , children_of : B , }
    };
}

WalkTreePrefix!();