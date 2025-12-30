// Generated macro for impl_23 (impl)
macro_rules! Depcrate_muteximpl_23 {
() => {
// Module: crate::mutex
// Provides: {"impl_23"}
// Dependencies: {}
impl < R : RawMutex , T > Mutex < R , T > { # [doc = " Creates a new mutex in an unlocked state ready for use."] # [inline] pub const fn new (val : T) -> Mutex < R , T > { Mutex { raw : R :: INIT , data : UnsafeCell :: new (val) , } } # [doc = " Consumes this mutex, returning the underlying data."] # [inline] pub fn into_inner (self) -> T { self . data . into_inner () } }
};
}
