// Generated macro for impl_37 (impl)
macro_rules! Depcrate_muteximpl_37 {
() => {
// Module: crate::mutex
// Provides: {"impl_37"}
// Dependencies: {}
impl < T > Mutex < T > { const_fn ! { const_if : # [cfg (not (loom))] ; # [doc = " Creates a new async mutex."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use async_lock::Mutex;"] # [doc = ""] # [doc = " let mutex = Mutex::new(0);"] # [doc = " ```"] pub const fn new (data : T) -> Mutex < T > { Mutex { state : AtomicUsize :: new (0) , lock_ops : Event :: new () , data : UnsafeCell :: new (data) , } } } # [doc = " Consumes the mutex, returning the underlying data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use async_lock::Mutex;"] # [doc = ""] # [doc = " let mutex = Mutex::new(10);"] # [doc = " assert_eq!(mutex.into_inner(), 10);"] # [doc = " ```"] pub fn into_inner (self) -> T { self . data . into_inner () } }
};
}
