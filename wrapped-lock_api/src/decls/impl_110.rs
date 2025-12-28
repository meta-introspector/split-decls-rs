macro_rules! deps {
    () => {
        RawRwLock!();
        RwLock!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < R : RawRwLock , T > RwLock < R , T > { # [doc = " Creates a new instance of an `RwLock<T>` which is unlocked."] # [inline] pub const fn new (val : T) -> RwLock < R , T > { RwLock { data : UnsafeCell :: new (val) , raw : R :: INIT , } } # [doc = " Consumes this `RwLock`, returning the underlying data."] # [inline] # [allow (unused_unsafe)] pub fn into_inner (self) -> T { unsafe { self . data . into_inner () } } }
    };
}

impl_110!();