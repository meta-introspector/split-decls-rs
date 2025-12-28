macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        unsafe impl < T , A > Sync for OccupiedEntry < '_ , T , A > where T : Sync , A : Sync + Allocator , { }
    };
}

impl_482!()