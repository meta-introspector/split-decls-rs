// Generated macro for impl_153 (impl)
macro_rules! Depcrate_arcimpl_153 {
() => {
// Module: crate::arc
// Provides: {"impl_153"}
// Dependencies: {}
impl < T : ? Sized > Arc < T > { # [inline] fn inner (& self) -> & ArcInner < T > { unsafe { & * self . ptr () } } # [inline (never)] unsafe fn drop_slow (& mut self) { unsafe { let _ = Box :: from_raw (self . ptr ()) ; } } # [doc = " Test pointer equality between the two Arcs, i.e. they must be the _same_"] # [doc = " allocation"] # [inline] pub (crate) fn ptr_eq (this : & Self , other : & Self) -> bool { std :: ptr :: addr_eq (this . ptr () , other . ptr ()) } pub (crate) fn ptr (& self) -> * mut ArcInner < T > { self . p . as_ptr () } }
};
}
