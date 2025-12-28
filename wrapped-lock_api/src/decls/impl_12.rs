macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < R , T > Mutex < R , T > { # [doc = " Creates a new mutex based on a pre-existing raw mutex."] # [inline] pub const fn from_raw (raw_mutex : R , val : T) -> Mutex < R , T > { Mutex { raw : raw_mutex , data : UnsafeCell :: new (val) , } } # [doc = " Creates a new mutex based on a pre-existing raw mutex."] # [doc = ""] # [doc = " This allows creating a mutex in a constant context on stable Rust."] # [doc = ""] # [doc = " This method is a legacy alias for [`from_raw`](Self::from_raw)."] # [inline] pub const fn const_new (raw_mutex : R , val : T) -> Mutex < R , T > { Self :: from_raw (raw_mutex , val) } }
    };
}

impl_12!()