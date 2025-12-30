// Generated macro for InOutBuf (struct)
macro_rules! Depcrate_inout_bufInOutBuf {
() => {
// Module: crate::inout_buf
// Provides: {"InOutBuf"}
// Dependencies: {}
# [doc = " Custom slice type which references one immutable (input) slice and one"] # [doc = " mutable (output) slice of equal length. Input and output slices are"] # [doc = " either the same or do not overlap."] pub struct InOutBuf < 'inp , 'out , T > { pub (crate) in_ptr : * const T , pub (crate) out_ptr : * mut T , pub (crate) len : usize , pub (crate) _pd : PhantomData < (& 'inp T , & 'out mut T) > , }
};
}
