// Generated macro for impl_2627 (impl)
macro_rules! Depcrate_lock_muteximpl_2627 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2627"}
// Dependencies: {}
impl < T > Mutex < T > { # [doc = " Creates a new futures-aware mutex."] pub const fn new (t : T) -> Self { Self { state : AtomicUsize :: new (0) , waiters : StdMutex :: new (Slab :: new ()) , value : UnsafeCell :: new (t) , } } # [doc = " Consumes this mutex, returning the underlying data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::lock::Mutex;"] # [doc = ""] # [doc = " let mutex = Mutex::new(0);"] # [doc = " assert_eq!(mutex.into_inner(), 0);"] # [doc = " ```"] pub fn into_inner (self) -> T { self . value . into_inner () } }
};
}
