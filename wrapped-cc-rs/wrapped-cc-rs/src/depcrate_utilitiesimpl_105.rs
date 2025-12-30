// Generated macro for impl_105 (impl)
macro_rules! Depcrate_utilitiesimpl_105 {
() => {
// Module: crate::utilities
// Provides: {"impl_105"}
// Dependencies: {}
impl < T > OnceLock < T > { pub (crate) const fn new () -> Self { Self { once : Once :: new () , value : UnsafeCell :: new (MaybeUninit :: uninit ()) , _marker : PhantomData , } } # [inline] fn is_initialized (& self) -> bool { self . once . is_completed () } unsafe fn get_unchecked (& self) -> & T { debug_assert ! (self . is_initialized ()) ; # [allow (clippy :: needless_borrow)] # [allow (unused_unsafe)] unsafe { (& * self . value . get ()) . assume_init_ref () } } pub (crate) fn get_or_init (& self , f : impl FnOnce () -> T) -> & T { self . once . call_once (| | { unsafe { & mut * self . value . get () } . write (f ()) ; }) ; unsafe { self . get_unchecked () } } pub (crate) fn get (& self) -> Option < & T > { if self . is_initialized () { Some (unsafe { self . get_unchecked () }) } else { None } } }
};
}
