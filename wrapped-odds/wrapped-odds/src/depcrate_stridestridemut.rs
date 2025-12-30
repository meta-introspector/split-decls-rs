// Generated macro for StrideMut (struct)
macro_rules! Depcrate_strideStrideMut {
() => {
// Module: crate::stride
// Provides: {"StrideMut"}
// Dependencies: {}
# [doc = " The mutable equivalent of Stride."] # [doc = ""] # [doc = " `StrideMut` does not support zero-sized types for `A`."] # [doc = ""] # [doc = " Iterator element type is `&'a mut A`."] pub struct StrideMut < 'a , A : 'a > { begin : * mut A , offset : isize , end : isize , stride : isize , life : marker :: PhantomData < & 'a mut A > , }
};
}
