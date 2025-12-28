macro_rules! TreeWalkMode {
    () => {
        # [doc = " A binary indicator of whether a tree walk should be performed in pre-order"] # [doc = " or post-order."] # [derive (Clone , Copy)] pub enum TreeWalkMode { # [doc = " Runs the traversal in pre-order."] PreOrder = 0 , # [doc = " Runs the traversal in post-order."] PostOrder = 1 , }
    };
}

TreeWalkMode!()