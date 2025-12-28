macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [cfg (feature = "stable_deref_trait")] unsafe impl stable_deref_trait :: StableDeref for Mmap { }
    };
}

impl_20!();