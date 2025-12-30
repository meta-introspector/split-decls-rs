// Generated macro for impl_119 (impl)
macro_rules! Depcrate_sync_once_lockimpl_119 {
() => {
// Module: crate::sync::once_lock
// Provides: {"impl_119"}
// Dependencies: {}
impl < T > OnceLock < T > { # [doc = " Creates a new empty cell."] # [must_use] pub (crate) const fn new () -> Self { Self { once : Once :: new () , value : UnsafeCell :: new (MaybeUninit :: uninit ()) , } } # [doc = " Gets the contents of the cell, initializing it with `f` if the cell"] # [doc = " was empty."] # [doc = ""] # [doc = " Many threads may call `get_or_init` concurrently with different"] # [doc = " initializing functions, but it is guaranteed that only one function"] # [doc = " will be executed."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `f` panics, the panic is propagated to the caller, and the cell"] # [doc = " remains uninitialized."] # [doc = ""] # [doc = " It is an error to reentrantly initialize the cell from `f`. The"] # [doc = " exact outcome is unspecified. Current implementation deadlocks, but"] # [doc = " this may be changed to a panic in the future."] pub (crate) fn get_or_init < F > (& self , f : F) -> & T where F : FnOnce () -> T , { if self . once . is_completed () { return unsafe { self . get_unchecked () } ; } self . initialize (f) ; unsafe { self . get_unchecked () } } # [cold] fn initialize < F > (& self , f : F) where F : FnOnce () -> T , { let slot = self . value . get () ; self . once . call_once (| | { let value = f () ; unsafe { slot . write (MaybeUninit :: new (value)) } }) ; } # [doc = " # Safety"] # [doc = ""] # [doc = " The value must be initialized"] unsafe fn get_unchecked (& self) -> & T { debug_assert ! (self . once . is_completed ()) ; unsafe { (* self . value . get ()) . assume_init_ref () } } }
};
}
