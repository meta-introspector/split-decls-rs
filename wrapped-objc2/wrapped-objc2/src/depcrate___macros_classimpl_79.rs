// Generated macro for impl_79 (impl)
macro_rules! Depcrate___macros_classimpl_79 {
() => {
// Module: crate::__macros::class
// Provides: {"impl_79"}
// Dependencies: {}
impl CachedClass { # [doc = " Constructs a new [`CachedClass`]."] # [allow (clippy :: new_without_default)] pub const fn new () -> CachedClass { CachedClass { ptr : AtomicPtr :: new (ptr :: null_mut ()) , } } # [cold] # [track_caller] unsafe fn fetch (& self , name : * const c_char) -> & 'static AnyClass { let ptr : * const AnyClass = unsafe { ffi :: objc_getClass (name) } . cast () ; self . ptr . store (ptr as * mut AnyClass , Ordering :: Relaxed) ; if let Some (cls) = unsafe { ptr . as_ref () } { cls } else { let name = unsafe { CStr :: from_ptr (name) } ; let name = str :: from_utf8 (name . to_bytes ()) . unwrap () ; panic ! ("class {name} could not be found") } } # [doc = " Returns the cached class. If no class is yet cached, gets one with"] # [doc = " the given name and stores it."] # [inline] # [track_caller] pub unsafe fn get (& self , name : & str) -> & 'static AnyClass { let ptr = self . ptr . load (Ordering :: Relaxed) ; if let Some (cls) = unsafe { ptr . as_ref () } { cls } else { unsafe { self . fetch (name . as_ptr () . cast ()) } } } }
};
}
