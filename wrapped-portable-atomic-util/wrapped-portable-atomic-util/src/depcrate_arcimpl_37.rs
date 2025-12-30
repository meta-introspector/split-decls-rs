// Generated macro for impl_37 (impl)
macro_rules! Depcrate_arcimpl_37 {
() => {
// Module: crate::arc
// Provides: {"impl_37"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_alloc_layout_extras))] impl < T > Arc < [T] > { # [doc = " Allocates an `ArcInner<[T]>` with the given length."] unsafe fn allocate_for_slice (len : usize) -> * mut ArcInner < [T] > { unsafe { Self :: allocate_for_layout (Layout :: array :: < T > (len) . unwrap () , | layout | Global . allocate (layout) , | mem | ptr :: slice_from_raw_parts_mut (mem . cast :: < T > () , len) as * mut ArcInner < [T] > ,) } } # [doc = " Constructs an `Arc<[T]>` from an iterator known to be of a certain size."] # [doc = ""] # [doc = " Behavior is undefined should the size be wrong."] unsafe fn from_iter_exact (iter : impl Iterator < Item = T > , len : usize) -> Self { struct Guard < T > { ptr : * mut ArcInner < [mem :: MaybeUninit < T >] > , elems : * mut T , n_elems : usize , } impl < T > Drop for Guard < T > { fn drop (& mut self) { unsafe { let slice = ptr :: slice_from_raw_parts_mut (self . elems , self . n_elems) ; ptr :: drop_in_place (slice) ; drop (Box :: from_raw (self . ptr)) ; } } } unsafe { let ptr : * mut ArcInner < [mem :: MaybeUninit < T >] > = Arc :: allocate_for_slice (len) ; let elems = (* ptr) . data . as_mut_ptr () as * mut T ; let mut guard = Guard { ptr , elems , n_elems : 0 } ; for (i , item) in iter . enumerate () { ptr :: write (elems . add (i) , item) ; guard . n_elems += 1 ; } mem :: forget (guard) ; Arc :: from_ptr (ptr) . assume_init () } } }
};
}
