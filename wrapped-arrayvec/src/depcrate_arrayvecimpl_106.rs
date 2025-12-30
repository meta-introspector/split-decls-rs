// Generated macro for impl_106 (impl)
macro_rules! Depcrate_arrayvecimpl_106 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_106"}
// Dependencies: {}
impl < T , const CAP : usize > ArrayVec < T , CAP > { # [doc = " Extend the arrayvec from the iterable."] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " Unsafe because if CHECK is false, the length of the input is not checked."] # [doc = " The caller must ensure the length of the input fits in the capacity."] # [track_caller] pub (crate) unsafe fn extend_from_iter < I , const CHECK : bool > (& mut self , iterable : I) where I : IntoIterator < Item = T > , { let take = self . capacity () - self . len () ; let len = self . len () ; let mut ptr = raw_ptr_add (self . as_mut_ptr () , len) ; let end_ptr = raw_ptr_add (ptr , take) ; let mut guard = ScopeExitGuard { value : & mut self . len , data : len , f : move | & len , self_len | { * * self_len = len as LenUint ; } , } ; let mut iter = iterable . into_iter () ; loop { if let Some (elt) = iter . next () { if ptr == end_ptr && CHECK { extend_panic () ; } debug_assert_ne ! (ptr , end_ptr) ; if mem :: size_of :: < T > () != 0 { ptr . write (elt) ; } ptr = raw_ptr_add (ptr , 1) ; guard . data += 1 ; } else { return ; } } } # [doc = " Extend the ArrayVec with clones of elements from the slice;"] # [doc = " the length of the slice must be <= the remaining capacity in the arrayvec."] pub (crate) fn extend_from_slice (& mut self , slice : & [T]) where T : Clone , { let take = self . capacity () - self . len () ; debug_assert ! (slice . len () <= take) ; unsafe { let slice = if take < slice . len () { & slice [.. take] } else { slice } ; self . extend_from_iter :: < _ , false > (slice . iter () . cloned ()) ; } } }
};
}
