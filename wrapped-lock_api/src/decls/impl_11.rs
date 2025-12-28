macro_rules! deps {
    () => {
        RawMutex!();
        Mutex!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < R : RawMutex , T > Mutex < R , T > { # [doc = " Creates a new mutex in an unlocked state ready for use."] # [inline] pub const fn new (val : T) -> Mutex < R , T > { Mutex { raw : R :: INIT , data : UnsafeCell :: new (val) , } } # [doc = " Consumes this mutex, returning the underlying data."] # [inline] pub fn into_inner (self) -> T { self . data . into_inner () } }
    };
}

impl_11!()