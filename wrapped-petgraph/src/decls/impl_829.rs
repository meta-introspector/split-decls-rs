macro_rules! deps {
    () => {
        EdgeReference!();
        IndexType!();
        EdgeType!();
        EdgesConnecting!();
    };
}

macro_rules! impl_829 {
    () => {
        deps!();
        impl < 'a , E , Ty , Ix > Iterator for EdgesConnecting < 'a , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Item = EdgeReference < 'a , E , Ix > ; fn next (& mut self) -> Option < EdgeReference < 'a , E , Ix > > { let target_node = self . target_node ; self . edges . by_ref () . find (| & edge | edge . node [1] == target_node) } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . edges . size_hint () ; (0 , upper) } }
    };
}

impl_829!()