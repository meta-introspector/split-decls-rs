macro_rules! deps {
    () => {
        OwnedMutexLockFuture!();
    };
}

macro_rules! impl_1295 {
    () => {
        deps!();
        impl < T : ? Sized > Drop for OwnedMutexLockFuture < T > { fn drop (& mut self) { if let Some (mutex) = self . mutex . as_ref () { mutex . remove_waker (self . wait_key , true) ; } } }
    };
}

impl_1295!();