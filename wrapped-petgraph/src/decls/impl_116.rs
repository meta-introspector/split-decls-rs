macro_rules! deps {
    () => {
        FilterNode!();
        VisitMap!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < N , S > FilterNode < N > for & HashSet < N , S > where HashSet < N , S > : VisitMap < N > , { fn include_node (& self , n : N) -> bool { self . is_visited (& n) } }
    };
}

impl_116!();