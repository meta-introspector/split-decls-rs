macro_rules! deps {
    () => {
        BiLockGuard!();
        BiLockAcquire!();
    };
}

macro_rules! impl_1277 {
    () => {
        deps!();
        # [cfg (feature = "bilock")] impl < 'a , T > Future for BiLockAcquire < 'a , T > { type Output = BiLockGuard < 'a , T > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . bilock . poll_lock (cx) } }
    };
}

impl_1277!()