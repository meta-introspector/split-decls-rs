// Generated macro for impl_412 (impl)
macro_rules! Depcrate_memmemimpl_412 {
() => {
// Module: crate::memmem
// Provides: {"impl_412"}
// Dependencies: {}
impl < 'h , 'n > FindRevIter < 'h , 'n > { # [inline (always)] pub (crate) fn new (haystack : & 'h [u8] , finder : FinderRev < 'n > ,) -> FindRevIter < 'h , 'n > { let pos = Some (haystack . len ()) ; FindRevIter { haystack , finder , pos } } # [doc = " Convert this iterator into its owned variant, such that it no longer"] # [doc = " borrows the finder and needle."] # [doc = ""] # [doc = " If this is already an owned iterator, then this is a no-op. Otherwise,"] # [doc = " this copies the needle."] # [doc = ""] # [doc = " This is only available when the `std` feature is enabled."] # [cfg (feature = "alloc")] # [inline] pub fn into_owned (self) -> FindRevIter < 'h , 'static > { FindRevIter { haystack : self . haystack , finder : self . finder . into_owned () , pos : self . pos , } } }
};
}
