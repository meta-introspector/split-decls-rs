macro_rules! deps {
    () => {
        MutexLockFuture!();
    };
}

macro_rules! impl_1303 {
    () => {
        deps!();
        impl < T : ? Sized > FusedFuture for MutexLockFuture < '_ , T > { fn is_terminated (& self) -> bool { self . mutex . is_none () } }
    };
}

impl_1303!()