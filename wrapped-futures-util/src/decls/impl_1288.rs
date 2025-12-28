macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_1288 {
    () => {
        deps!();
        impl < T > Mutex < T > { # [doc = " Creates a new futures-aware mutex."] pub const fn new (t : T) -> Self { Self { state : AtomicUsize :: new (0) , waiters : StdMutex :: new (Slab :: new ()) , value : UnsafeCell :: new (t) , } } # [doc = " Consumes this mutex, returning the underlying data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::lock::Mutex;"] # [doc = ""] # [doc = " let mutex = Mutex::new(0);"] # [doc = " assert_eq!(mutex.into_inner(), 0);"] # [doc = " ```"] pub fn into_inner (self) -> T { self . value . into_inner () } }
    };
}

impl_1288!();