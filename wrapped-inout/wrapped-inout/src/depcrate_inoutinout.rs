// Generated macro for InOut (struct)
macro_rules! Depcrate_inoutInOut {
() => {
// Module: crate::inout
// Provides: {"InOut"}
// Dependencies: {}
# [doc = " Custom pointer type which contains one immutable (input) and one mutable"] # [doc = " (output) pointer, which are either equal or non-overlapping."] pub struct InOut < 'inp , 'out , T > { pub (crate) in_ptr : * const T , pub (crate) out_ptr : * mut T , pub (crate) _pd : PhantomData < (& 'inp T , & 'out mut T) > , }
};
}
