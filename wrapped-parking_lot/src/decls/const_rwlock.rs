macro_rules! deps {
    () => {
        RwLock!();
        RawRwLock!();
    };
}

macro_rules! const_rwlock {
    () => {
        deps!();
        # [doc = " Creates a new instance of an `RwLock<T>` which is unlocked."] # [doc = ""] # [doc = " This allows creating a `RwLock<T>` in a constant context on stable Rust."] pub const fn const_rwlock < T > (val : T) -> RwLock < T > { RwLock :: const_new (< RawRwLock as lock_api :: RawRwLock > :: INIT , val) }
    };
}

const_rwlock!()