// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl NonMaxUsize { # [doc = " Convert an index to a usize"] # [cfg_attr (mutants , mutants :: skip)] # [inline] const fn get (& self) -> usize { self . 0 . get () - 1 } # [doc = " Create a new index from a usize, if `index` is `usize::MAX` then `None` is returned"] # [inline] const fn new (index : usize) -> Option < Self > { match NonZeroUsize :: new (index . wrapping_add (1)) { Some (index) => Some (Self (index)) , None => None , } } # [doc = " Create a new index from a usize, without checking if `index` is `usize::MAX`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `index` must not be `usize::MAX`"] # [cfg (feature = "std")] # [inline] const unsafe fn new_unchecked (index : usize) -> Self { Self (unsafe { NonZeroUsize :: new_unchecked (index + 1) }) } # [doc = " Add an unsigned integer to a index. Check for bound violation and return `None` if the result will be larger than or equal to `usize::MAX`"] # [cfg (feature = "std")] # [inline] fn checked_add (& self , rhs : usize) -> Option < Self > { self . 0 . checked_add (rhs) . map (Self) } # [doc = " Subtract an unsigned integer from a index. Check for bound violation and return `None` if the result will be less than 0."] # [cfg (feature = "std")] # [inline] fn checked_sub (& self , rhs : usize) -> Option < Self > { self . get () . checked_sub (rhs) . map (| i | unsafe { Self :: new_unchecked (i) }) } # [cfg (feature = "std")] # [inline] const fn zero () -> Self { Self (unsafe { NonZeroUsize :: new_unchecked (1) }) } }
};
}
