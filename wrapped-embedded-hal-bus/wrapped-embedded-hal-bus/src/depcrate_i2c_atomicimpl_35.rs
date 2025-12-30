// Generated macro for impl_35 (impl)
macro_rules! Depcrate_i2c_atomicimpl_35 {
() => {
// Module: crate::i2c::atomic
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a , T > AtomicDevice < 'a , T > where T : I2c , { # [doc = " Create a new `AtomicDevice`."] # [inline] pub fn new (bus : & 'a AtomicCell < T >) -> Self { Self { bus } } fn lock < R , F > (& self , f : F) -> Result < R , AtomicError < T :: Error > > where F : FnOnce (& mut T) -> Result < R , < T as ErrorType > :: Error > , { self . bus . busy . compare_exchange (false , true , core :: sync :: atomic :: Ordering :: SeqCst , core :: sync :: atomic :: Ordering :: SeqCst ,) . map_err (| _ | AtomicError :: < T :: Error > :: Busy) ? ; let result = f (unsafe { & mut * self . bus . bus . get () }) ; self . bus . busy . store (false , core :: sync :: atomic :: Ordering :: SeqCst) ; result . map_err (AtomicError :: Other) } }
};
}
