macro_rules! deps {
    () => {
        Kind!();
        LenType!();
        FindMutView!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl < T , Idx , K > DerefMut for FindMutView < '_ , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { fn deref_mut (& mut self) -> & mut Self :: Target { self . maybe_changed = true ; self . list . read_mut_data_in_node_at (self . index . into_usize ()) } }
    };
}

impl_445!();