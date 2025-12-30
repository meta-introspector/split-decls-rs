// Generated macro for Array (struct)
macro_rules! Depcrate_atomicArray {
() => {
// Module: crate::atomic
// Provides: {"Array"}
// Dependencies: {}
# [doc = " Array with size."] # [doc = ""] # [doc = " # Memory layout"] # [doc = ""] # [doc = " An array consisting of size and elements:"] # [doc = ""] # [doc = " ```text"] # [doc = "          elements"] # [doc = "          |"] # [doc = "          |"] # [doc = " ------------------------------------"] # [doc = " | size | 0 | 1 | 2 | 3 | 4 | 5 | 6 |"] # [doc = " ------------------------------------"] # [doc = " ```"] # [doc = ""] # [doc = " Its memory layout is different from that of `Box<[T]>` in that size is in the allocation (not"] # [doc = " along with pointer as in `Box<[T]>`)."] # [doc = ""] # [doc = " Elements are not present in the type, but they will be in the allocation."] # [repr (C)] struct Array < T > { # [doc = " The number of elements (not the number of bytes)."] len : usize , elements : [MaybeUninit < T > ; 0] , }
};
}
