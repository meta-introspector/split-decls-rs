macro_rules! deps {
    () => {
        FairTimeout!();
        WordLock!();
        Bucket!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Bucket { # [inline] pub fn new (timeout : TimeoutInstant , seed : u32) -> Self { Self { mutex : WordLock :: new () , queue_head : Cell :: new (ptr :: null ()) , queue_tail : Cell :: new (ptr :: null ()) , fair_timeout : UnsafeCell :: new (FairTimeout :: new (timeout , seed)) , } } }
    };
}

impl_7!();