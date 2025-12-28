macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! WalkTreePostfix {
    () => {
        deps!();
        # [doc = " ParallelIterator for arbitrary tree-shaped patterns."] # [doc = " Returned by the [`walk_tree_postfix()`] function."] # [derive (Debug)] pub struct WalkTreePostfix < S , B > { initial_state : S , children_of : B , }
    };
}

WalkTreePostfix!()