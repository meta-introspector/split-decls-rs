macro_rules! deps {
    () => {
        Latch!();
        LockLatch!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Latch for LockLatch { # [inline] unsafe fn set (this : * const Self) { unsafe { let mut guard = (* this) . m . lock () . unwrap () ; * guard = true ; (* this) . v . notify_all () ; } } }
    };
}

impl_84!()