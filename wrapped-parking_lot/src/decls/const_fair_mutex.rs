macro_rules! deps {
    () => {
        FairMutex!();
        RawMutex!();
        RawFairMutex!();
    };
}

macro_rules! const_fair_mutex {
    () => {
        deps!();
        # [doc = " Creates a new fair mutex in an unlocked state ready for use."] # [doc = ""] # [doc = " This allows creating a fair mutex in a constant context on stable Rust."] pub const fn const_fair_mutex < T > (val : T) -> FairMutex < T > { FairMutex :: const_new (< RawFairMutex as lock_api :: RawMutex > :: INIT , val) }
    };
}

const_fair_mutex!();