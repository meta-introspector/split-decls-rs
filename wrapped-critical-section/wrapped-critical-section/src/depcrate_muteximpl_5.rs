// Generated macro for impl_5 (impl)
macro_rules! Depcrate_muteximpl_5 {
() => {
// Module: crate::mutex
// Provides: {"impl_5"}
// Dependencies: {}
impl < T > Mutex < T > { # [doc = " Creates a new mutex."] # [inline] pub const fn new (value : T) -> Self { Mutex { inner : UnsafeCell :: new (value) , } } # [doc = " Gets a mutable reference to the contained value when the mutex is already uniquely borrowed."] # [doc = ""] # [doc = " This does not require locking or a critical section since it takes `&mut self`, which"] # [doc = " guarantees unique ownership already. Care must be taken when using this method to"] # [doc = " **unsafely** access `static mut` variables, appropriate fences must be used to prevent"] # [doc = " unwanted optimizations."] # [inline] pub fn get_mut (& mut self) -> & mut T { unsafe { & mut * self . inner . get () } } # [doc = " Unwraps the contained value, consuming the mutex."] # [inline] pub fn into_inner (self) -> T { self . inner . into_inner () } # [doc = " Borrows the data for the duration of the critical section."] # [inline] pub fn borrow < 'cs > (& 'cs self , _cs : CriticalSection < 'cs >) -> & 'cs T { unsafe { & * self . inner . get () } } }
};
}
