macro_rules! deps {
    () => {
        FilterNode!();
        VisitMap!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        # [doc = " This filter includes the nodes that are contained in the set."] impl < N , S > FilterNode < N > for HashSet < N , S > where HashSet < N , S > : VisitMap < N > , { fn include_node (& self , n : N) -> bool { self . is_visited (& n) } }
    };
}

impl_114!()