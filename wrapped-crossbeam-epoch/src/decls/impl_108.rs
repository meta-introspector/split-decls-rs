macro_rules! deps {
    () => {
        SealedBag!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        # [doc = " It is safe to share `SealedBag` because `is_expired` only inspects the epoch."] unsafe impl Sync for SealedBag { }
    };
}

impl_108!();