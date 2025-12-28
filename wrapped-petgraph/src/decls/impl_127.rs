macro_rules! deps {
    () => {
        NodeRef!();
        NodeFilteredNodes!();
        FilterNode!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < I , F > Iterator for NodeFilteredNodes < '_ , I , F > where I : Iterator , I :: Item : Copy + NodeRef , F : FilterNode < < I :: Item as NodeRef > :: NodeId > , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let f = self . f ; if ! self . include_source { None } else { self . iter . find (move | & target | f . include_node (target . id ())) } } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
    };
}

impl_127!();