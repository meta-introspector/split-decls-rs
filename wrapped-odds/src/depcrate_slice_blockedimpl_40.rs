// Generated macro for impl_40 (impl)
macro_rules! Depcrate_slice_blockedimpl_40 {
() => {
// Module: crate::slice::blocked
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a , B , T > BlockedIter < 'a , B , T > where B : Block < Item = T > , { # [doc = " Create an `BlockedIter` out of the slice of data, which iterates first"] # [doc = " in blocks of `T`, and then leaves a tail of the remaining"] # [doc = " elements."] pub fn from_slice (data : & 'a [T]) -> Self { assert ! (size_of ::< T > () != 0) ; unsafe { let ptr = data . as_ptr () ; let len = data . len () ; let end = ptr . offset (len as isize) ; BlockedIter { ptr : ptr , end : end , ty1 : PhantomData , ty2 : PhantomData , } } } # [doc = " Return an iterator of the remaining tail;"] # [doc = " this can be called at any time, but in particular when the iterator"] # [doc = " has returned None."] # [inline (always)] pub fn tail (& self) -> SliceIter < 'a , T > { unsafe { SliceIter :: new (self . ptr , self . end) } } # [doc = " Return `true` if the tail is not empty."] pub fn has_tail (& self) -> bool { self . ptr != self . end } # [doc = " Return the next iterator element, without stepping the iterator."] pub fn peek_next (& self) -> Option < < Self as Iterator > :: Item > { if ptrdistance (self . ptr , self . end) >= B :: capacity () { unsafe { Some (& * (self . ptr as * const B)) } } else { None } } }
};
}
