macro_rules! deps {
    () => {
        FilterNode!();
        NodeFilteredNeighbors!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < I , F > Iterator for NodeFilteredNeighbors < '_ , I , F > where I : Iterator , I :: Item : Copy , F : FilterNode < I :: Item > , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let f = self . f ; if ! self . include_source { None } else { self . iter . find (move | & target | f . include_node (target)) } } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
    };
}

impl_122!();