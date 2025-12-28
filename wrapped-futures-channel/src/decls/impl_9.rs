macro_rules! deps {
    () => {
        TryLock!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < T > Drop for TryLock < '_ , T > { fn drop (& mut self) { self . __ptr . locked . store (false , SeqCst) ; } }
    };
}

impl_9!();