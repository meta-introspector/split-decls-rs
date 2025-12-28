macro_rules! deps {
    () => {
        LenType!();
        Kind!();
        FindMutView!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        impl < T , Idx , K > Deref for FindMutView < '_ , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { type Target = T ; fn deref (& self) -> & Self :: Target { self . list . read_data_in_node_at (self . index . into_usize ()) } }
    };
}

impl_444!();