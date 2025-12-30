// Generated macro for array_assume_init (function)
macro_rules! Depcrate_utils_arrayarray_assume_init {
() => {
// Module: crate::utils::array
// Provides: {"array_assume_init"}
// Dependencies: {}
# [doc = " Extracts the values from an array of `MaybeUninit` containers."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " It is up to the caller to guarantee that all elements of the array are"] # [doc = " in an initialized state."] # [doc = ""] # [doc = " Inlined version of: <https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#method.array_assume_init>"] pub (crate) unsafe fn array_assume_init < T , const N : usize > (array : [MaybeUninit < T > ; N]) -> [T ; N] { let ret = unsafe { (& array as * const _ as * const [T ; N]) . read () } ; # [allow (clippy :: forget_non_drop)] mem :: forget (array) ; ret }
};
}
