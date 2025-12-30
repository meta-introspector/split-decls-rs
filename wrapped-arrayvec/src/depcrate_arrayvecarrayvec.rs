// Generated macro for ArrayVec (struct)
macro_rules! Depcrate_arrayvecArrayVec {
() => {
// Module: crate::arrayvec
// Provides: {"ArrayVec"}
// Dependencies: {}
# [doc = " A vector with a fixed capacity."] # [doc = ""] # [doc = " The `ArrayVec` is a vector backed by a fixed size array. It keeps track of"] # [doc = " the number of initialized elements. The `ArrayVec<T, CAP>` is parameterized"] # [doc = " by `T` for the element type and `CAP` for the maximum capacity."] # [doc = ""] # [doc = " `CAP` is of type `usize` but is range limited to `u32::MAX` (or `u16::MAX` on 16-bit targets);"] # [doc = " attempting to create larger arrayvecs with larger capacity will panic."] # [doc = ""] # [doc = " The vector is a contiguous value (storing the elements inline) that you can store directly on"] # [doc = " the stack if needed."] # [doc = ""] # [doc = " It offers a simple API but also dereferences to a slice, so that the full slice API is"] # [doc = " available. The ArrayVec can be converted into a by value iterator."] # [repr (C)] pub struct ArrayVec < T , const CAP : usize > { len : LenUint , xs : [MaybeUninit < T > ; CAP] , }
};
}
