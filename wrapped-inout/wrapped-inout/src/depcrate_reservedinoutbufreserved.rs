// Generated macro for InOutBufReserved (struct)
macro_rules! Depcrate_reservedInOutBufReserved {
() => {
// Module: crate::reserved
// Provides: {"InOutBufReserved"}
// Dependencies: {}
# [doc = " Custom slice type which references one immutable (input) slice and one"] # [doc = " mutable (output) slice. Input and output slices are either the same or"] # [doc = " do not overlap. Length of the output slice is always equal or bigger than"] # [doc = " length of the input slice."] pub struct InOutBufReserved < 'inp , 'out , T > { in_ptr : * const T , out_ptr : * mut T , in_len : usize , out_len : usize , _pd : PhantomData < (& 'inp T , & 'out mut T) > , }
};
}
