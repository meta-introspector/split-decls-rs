macro_rules! deps {
    () => {
        ItemTreeNode!();
    };
}

macro_rules! ItemTreeAstId {
    () => {
        deps!();
        # [allow (type_alias_bounds)] pub (crate) type ItemTreeAstId < T : ItemTreeNode > = FileAstId < T :: Source > ;
    };
}

ItemTreeAstId!()