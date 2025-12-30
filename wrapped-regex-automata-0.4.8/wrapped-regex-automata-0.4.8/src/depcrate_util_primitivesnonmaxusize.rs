// Generated macro for NonMaxUsize (struct)
macro_rules! Depcrate_util_primitivesNonMaxUsize {
() => {
// Module: crate::util::primitives
// Provides: {"NonMaxUsize"}
// Dependencies: {}
# [doc = " A `usize` that can never be `usize::MAX`."] # [doc = ""] # [doc = " This is similar to `core::num::NonZeroUsize`, but instead of not permitting"] # [doc = " a zero value, this does not permit a max value."] # [doc = ""] # [doc = " This is useful in certain contexts where one wants to optimize the memory"] # [doc = " usage of things that contain match offsets. Namely, since Rust slices"] # [doc = " are guaranteed to never have a length exceeding `isize::MAX`, we can use"] # [doc = " `usize::MAX` as a sentinel to indicate that no match was found. Indeed,"] # [doc = " types like `Option<NonMaxUsize>` have exactly the same size in memory as a"] # [doc = " `usize`."] # [doc = ""] # [doc = " This type is defined to be `repr(transparent)` for"] # [doc = " `core::num::NonZeroUsize`, which is in turn defined to be"] # [doc = " `repr(transparent)` for `usize`."] # [derive (Clone , Copy , Eq , Hash , PartialEq , PartialOrd , Ord)] # [repr (transparent)] pub struct NonMaxUsize (NonZeroUsize) ;
};
}
