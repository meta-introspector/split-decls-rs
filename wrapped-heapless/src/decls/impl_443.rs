macro_rules! deps {
    () => {
        Kind!();
        FindMutView!();
        LenType!();
    };
}

macro_rules! impl_443 {
    () => {
        deps!();
        impl < T , Idx , K > Drop for FindMutView < '_ , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { fn drop (& mut self) { if self . maybe_changed { let val = self . pop_internal () ; unsafe { self . list . push_unchecked (val) } ; } } }
    };
}

impl_443!();