macro_rules! deps {
    () => {
        BiLockAcquire!();
    };
}

macro_rules! impl_1276 {
    () => {
        deps!();
        # [cfg (feature = "bilock")] impl < T > Unpin for BiLockAcquire < '_ , T > { }
    };
}

impl_1276!()