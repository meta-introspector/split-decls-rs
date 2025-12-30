// Generated macro for impl_156 (impl)
macro_rules! Depcrate_arcimpl_156 {
() => {
// Module: crate::arc
// Provides: {"impl_156"}
// Dependencies: {}
impl < T : ? Sized > Arc < T > { # [doc = " Provides mutable access to the contents _if_ the `Arc` is uniquely owned."] # [inline] pub (crate) fn get_mut (this : & mut Self) -> Option < & mut T > { if this . is_unique () { unsafe { Some (& mut (* this . ptr ()) . data) } } else { None } } # [doc = " Whether or not the `Arc` is uniquely owned (is the refcount 1?)."] pub (crate) fn is_unique (& self) -> bool { self . inner () . count . load (Acquire) == 1 } }
};
}
