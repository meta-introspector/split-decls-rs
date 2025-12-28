macro_rules! deps {
    () => {
        RwLock!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < R , T > RwLock < R , T > { # [doc = " Creates a new new instance of an `RwLock<T>` based on a pre-existing"] # [doc = " `RawRwLock<T>`."] # [inline] pub const fn from_raw (raw_rwlock : R , val : T) -> RwLock < R , T > { RwLock { data : UnsafeCell :: new (val) , raw : raw_rwlock , } } # [doc = " Creates a new new instance of an `RwLock<T>` based on a pre-existing"] # [doc = " `RawRwLock<T>`."] # [doc = ""] # [doc = " This allows creating a `RwLock<T>` in a constant context on stable"] # [doc = " Rust."] # [doc = ""] # [doc = " This method is a legacy alias for [`from_raw`](Self::from_raw)."] # [inline] pub const fn const_new (raw_rwlock : R , val : T) -> RwLock < R , T > { Self :: from_raw (raw_rwlock , val) } }
    };
}

impl_111!();