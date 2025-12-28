macro_rules! deps {
    () => {
        JobRef!();
        LockLatch!();
        ThreadInfo!();
        OnceLatch!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl ThreadInfo { fn new (stealer : Stealer < JobRef >) -> ThreadInfo { ThreadInfo { primed : LockLatch :: new () , stopped : LockLatch :: new () , terminate : OnceLatch :: new () , stealer , } } }
    };
}

impl_120!();