macro_rules! deps {
    () => {
        List!();
        IndexType!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl < E , Ix : IndexType > visit :: NodeIndexable for List < E , Ix > { fn node_bound (& self) -> usize { self . node_count () } # [inline] fn to_index (& self , a : Self :: NodeId) -> usize { a . index () } # [inline] fn from_index (& self , i : usize) -> Self :: NodeId { Ix :: new (i) } }
    };
}

impl_318!()