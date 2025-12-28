macro_rules! deps {
    () => {
        MutexLockFuture!();
    };
}

macro_rules! impl_1305 {
    () => {
        deps!();
        impl < T : ? Sized > Drop for MutexLockFuture < '_ , T > { fn drop (& mut self) { if let Some (mutex) = self . mutex { mutex . remove_waker (self . wait_key , true) ; } } }
    };
}

impl_1305!();