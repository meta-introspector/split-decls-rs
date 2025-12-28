macro_rules! deps {
    () => {
        OwnedMutexLockFuture!();
    };
}

macro_rules! impl_1293 {
    () => {
        deps!();
        impl < T : ? Sized > FusedFuture for OwnedMutexLockFuture < T > { fn is_terminated (& self) -> bool { self . mutex . is_none () } }
    };
}

impl_1293!()