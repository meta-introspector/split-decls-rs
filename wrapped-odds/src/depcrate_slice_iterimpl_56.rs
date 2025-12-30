// Generated macro for impl_56 (impl)
macro_rules! Depcrate_slice_iterimpl_56 {
() => {
// Module: crate::slice::iter
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a , T > SliceCopyIter < 'a , T > where T : Copy , { # [doc = " Create a new slice copy iterator"] # [doc = ""] # [doc = " Panics if `T` is a zero-sized type. That case is not supported."] # [inline] pub unsafe fn new (ptr : * const T , end : * const T) -> Self { assert ! (size_of ::< T > () != 0) ; SliceCopyIter { ptr : ptr , end : end , ty : PhantomData , } } # [doc = " Return the start, end pointer of the iterator"] pub fn into_raw (self) -> (* const T , * const T) { (self . ptr , self . end) } # [doc = " Return the start pointer"] pub fn start (& self) -> * const T { self . ptr } # [doc = " Return the end pointer"] pub fn end (& self) -> * const T { self . end } # [doc = " Return mutable reference to the start pointer"] # [doc = ""] # [doc = " Unsafe because it is easy to violate memory safety by setting"] # [doc = " the pointer outside the data's valid range."] pub unsafe fn start_mut (& mut self) -> & mut * const T { & mut self . ptr } # [doc = " Return mutable reference to the start pointer"] # [doc = ""] # [doc = " Unsafe because it is easy to violate memory safety by setting"] # [doc = " the pointer outside the data's valid range."] pub unsafe fn end_mut (& mut self) -> & mut * const T { & mut self . end } # [doc = " Return the next iterator element, without stepping the iterator."] pub fn peek_next (& self) -> Option < < Self as Iterator > :: Item > { if self . ptr != self . end { unsafe { Some (* self . ptr) } } else { None } } }
};
}
