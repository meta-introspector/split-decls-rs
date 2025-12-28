macro_rules! deps {
    () => {
        InterpCx!();
        Machine!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl < 'tcx , M : Machine < 'tcx > > InterpCx < 'tcx , M > { # [doc = " This function is used by Miri's provenance GC to remove unreachable entries from the dead_alloc_map."] pub fn remove_unreachable_allocs (& mut self , reachable_allocs : & FxHashSet < AllocId >) { # [allow (rustc :: potential_query_instability)] self . memory . dead_alloc_map . retain (| id , _ | reachable_allocs . contains (id)) ; } }
    };
}

impl_254!();