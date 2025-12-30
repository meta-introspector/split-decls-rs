// Generated macro for impl_406 (impl)
macro_rules! Depcrate___macros_selimpl_406 {
() => {
// Module: crate::__macros::sel
// Provides: {"impl_406"}
// Dependencies: {}
impl CachedSel { # [doc = " Constructs a new [`CachedSel`]."] # [allow (clippy :: new_without_default)] pub const fn new () -> Self { Self { ptr : AtomicPtr :: new (ptr :: null_mut ()) , } } # [cold] unsafe fn fetch (& self , name : * const c_char) -> Sel { let sel = unsafe { Sel :: register_unchecked (name) } ; self . ptr . store (sel . as_ptr () as * mut _ , Ordering :: Relaxed) ; sel } # [doc = " Returns the cached selector. If no selector is yet cached, registers"] # [doc = " one with the given name and stores it."] # [inline] pub unsafe fn get (& self , name : & str) -> Sel { let ptr = self . ptr . load (Ordering :: Relaxed) ; if let Some (sel) = unsafe { Sel :: from_ptr (ptr) } { sel } else { unsafe { self . fetch (name . as_ptr () . cast ()) } } } }
};
}
