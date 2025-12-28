macro_rules! deps {
    () => {
        TreeNode!();
    };
}

macro_rules! TreeNodes {
    () => {
        deps!();
        # [doc = " The flat list of all tree-nodes so we can avoid having a linked-tree using pointers"] # [doc = " which is useful for traversal and initial setup as that can then trivially be non-recursive."] pub struct TreeNodes (Vec < TreeNode >) ;
    };
}

TreeNodes!()