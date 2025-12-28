macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl < T > Mutex < T > { # [doc = " Creates a new mutex in an unlocked state ready for use."] pub fn new (data : T) -> Mutex < T > { Mutex { data : std :: sync :: Mutex :: new (data) , object : rt :: Mutex :: new (true) , } } # [doc = " Consumes this mutex, returning the underlying data."] pub fn into_inner (self) -> LockResult < T > { Ok (self . data . into_inner () . unwrap ()) } }
    };
}

impl_297!();