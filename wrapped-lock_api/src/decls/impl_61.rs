macro_rules! deps {
    () => {
        RawReentrantMutex!();
        ReentrantMutex!();
        GetThreadId!();
        RawMutex!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < R : RawMutex , G : GetThreadId , T > ReentrantMutex < R , G , T > { # [doc = " Creates a new reentrant mutex in an unlocked state ready for use."] # [inline] pub const fn new (val : T) -> ReentrantMutex < R , G , T > { ReentrantMutex { data : UnsafeCell :: new (val) , raw : RawReentrantMutex { owner : AtomicUsize :: new (0) , lock_count : Cell :: new (0) , mutex : R :: INIT , get_thread_id : G :: INIT , } , } } # [doc = " Consumes this mutex, returning the underlying data."] # [inline] pub fn into_inner (self) -> T { self . data . into_inner () } }
    };
}

impl_61!()