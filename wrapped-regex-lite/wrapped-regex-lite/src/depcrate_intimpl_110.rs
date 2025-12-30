// Generated macro for impl_110 (impl)
macro_rules! Depcrate_intimpl_110 {
() => {
// Module: crate::int
// Provides: {"impl_110"}
// Dependencies: {}
impl NonMaxUsize { # [doc = " Create a new `NonMaxUsize` from the given value."] # [doc = ""] # [doc = " This returns `None` only when the given value is equal to `usize::MAX`."] pub (crate) fn new (value : usize) -> Option < NonMaxUsize > { NonZeroUsize :: new (value . wrapping_add (1)) . map (NonMaxUsize) } # [doc = " Return the underlying `usize` value. The returned value is guaranteed"] # [doc = " to not equal `usize::MAX`."] pub (crate) fn get (self) -> usize { self . 0 . get () . wrapping_sub (1) } }
};
}
