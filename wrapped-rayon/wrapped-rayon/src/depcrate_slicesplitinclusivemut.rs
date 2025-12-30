// Generated macro for SplitInclusiveMut (struct)
macro_rules! Depcrate_sliceSplitInclusiveMut {
() => {
// Module: crate::slice
// Provides: {"SplitInclusiveMut"}
// Dependencies: {}
# [doc = " Parallel iterator over mutable slices separated by a predicate,"] # [doc = " including the matched part as a terminator."] pub struct SplitInclusiveMut < 'data , T , P > { slice : & 'data mut [T] , separator : P , }
};
}
