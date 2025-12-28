macro_rules! deps {
    () => {
        MmapMut!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [cfg (feature = "stable_deref_trait")] unsafe impl stable_deref_trait :: StableDeref for MmapMut { }
    };
}

impl_31!();