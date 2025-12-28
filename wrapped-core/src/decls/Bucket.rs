macro_rules! deps {
    () => {
        WordLock!();
        ThreadData!();
        FairTimeout!();
    };
}

macro_rules! Bucket {
    () => {
        deps!();
        # [repr (align (64))] struct Bucket { mutex : WordLock , queue_head : Cell < * const ThreadData > , queue_tail : Cell < * const ThreadData > , fair_timeout : UnsafeCell < FairTimeout > , }
    };
}

Bucket!()