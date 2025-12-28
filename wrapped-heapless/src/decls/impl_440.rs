macro_rules! deps {
    () => {
        LenType!();
        IterView!();
        Kind!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        impl < 'a , T , Idx , K > Iterator for IterView < 'a , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { let index = self . index . to_non_max () ? ; let node = self . list . node_at (index) ; self . index = node . next ; Some (self . list . read_data_in_node_at (index)) } }
    };
}

impl_440!();