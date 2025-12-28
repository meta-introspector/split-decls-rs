macro_rules! deps {
    () => {
        PatField!();
    };
}

macro_rules! walk_flat_map_pat_field {
    () => {
        deps!();
        pub fn walk_flat_map_pat_field < T : MutVisitor > (vis : & mut T , mut fp : PatField ,) -> SmallVec < [PatField ; 1] > { vis . visit_pat_field (& mut fp) ; smallvec ! [fp] }
    };
}

walk_flat_map_pat_field!();