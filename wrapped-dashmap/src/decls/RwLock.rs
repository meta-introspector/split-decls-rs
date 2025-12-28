macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! RwLock {
    () => {
        deps!();
        pub type RwLock < T > = lock_api :: RwLock < RawRwLock , T > ;
    };
}

RwLock!()