macro_rules! deps {
    () => {
        IndexType!();
        EdgeIndex!();
        EdgeIndices!();
    };
}

macro_rules! impl_846 {
    () => {
        deps!();
        impl < E , Ix : IndexType > Iterator for EdgeIndices < '_ , E , Ix > { type Item = EdgeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . ex_find_map (| (i , node) | { if node . weight . is_some () { Some (edge_index (i)) } else { None } }) } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
    };
}

impl_846!()