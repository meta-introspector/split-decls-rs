macro_rules! deps {
    () => {
        Mutex!();
        RawMutex!();
    };
}

macro_rules! const_mutex {
    () => {
        deps!();
        # [doc = " Creates a new mutex in an unlocked state ready for use."] # [doc = ""] # [doc = " This allows creating a mutex in a constant context on stable Rust."] pub const fn const_mutex < T > (val : T) -> Mutex < T > { Mutex :: const_new (< RawMutex as lock_api :: RawMutex > :: INIT , val) }
    };
}

const_mutex!()