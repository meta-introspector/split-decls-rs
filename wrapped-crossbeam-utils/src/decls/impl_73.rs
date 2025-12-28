macro_rules! deps {
    () => {
        CachePadded!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        unsafe impl < T : Sync > Sync for CachePadded < T > { }
    };
}

impl_73!();