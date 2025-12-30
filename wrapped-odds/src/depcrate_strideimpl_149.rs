// Generated macro for impl_149 (impl)
macro_rules! Depcrate_strideimpl_149 {
() => {
// Module: crate::stride
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'a , A > Stride < 'a , A > { # [doc = " Create a Stride iterator from a raw pointer."] pub unsafe fn from_ptr_len (begin : * const A , nelem : usize , stride : isize) -> Stride < 'a , A > { Stride { begin : begin , offset : 0 , end : stride * nelem as isize , stride : stride , life : marker :: PhantomData , } } }
};
}
