macro_rules! deps {
    () => {
        WithDummy!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        impl < G : NodeIndexable > WithDummy for G { fn dummy_idx (& self) -> usize { self . node_bound () } fn try_from_index (& self , i : usize) -> Option < Self :: NodeId > { if i != self . dummy_idx () { Some (self . from_index (i)) } else { None } } }
    };
}

impl_402!()