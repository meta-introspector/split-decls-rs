macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! ItemTreeNode {
    () => {
        deps!();
        # [doc = " Trait implemented by all nodes in the item tree."] pub (crate) trait ItemTreeNode : Clone { type Source : AstIdNode ; }
    };
}

ItemTreeNode!();