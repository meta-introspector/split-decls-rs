// Generated macro for impl_115 (impl)
macro_rules! Depcrate_epochimpl_115 {
() => {
// Module: crate::epoch
// Provides: {"impl_115"}
// Dependencies: {}
impl Epoch { # [doc = " Returns the starting epoch in unpinned state."] # [inline] pub (crate) fn starting () -> Self { Self :: default () } # [doc = " Returns the number of epochs `self` is ahead of `rhs`."] # [doc = ""] # [doc = " Internally, epochs are represented as numbers in the range `(isize::MIN / 2) .. (isize::MAX"] # [doc = " / 2)`, so the returned distance will be in the same interval."] pub (crate) fn wrapping_sub (self , rhs : Self) -> isize { self . data . wrapping_sub (rhs . data & ! 1) as isize >> 1 } # [doc = " Returns `true` if the epoch is marked as pinned."] # [inline] pub (crate) fn is_pinned (self) -> bool { (self . data & 1) == 1 } # [doc = " Returns the same epoch, but marked as pinned."] # [inline] pub (crate) fn pinned (self) -> Self { Self { data : self . data | 1 , } } # [doc = " Returns the same epoch, but marked as unpinned."] # [inline] pub (crate) fn unpinned (self) -> Self { Self { data : self . data & ! 1 , } } # [doc = " Returns the successor epoch."] # [doc = ""] # [doc = " The returned epoch will be marked as pinned only if the previous one was as well."] # [inline] pub (crate) fn successor (self) -> Self { Self { data : self . data . wrapping_add (2) , } } }
};
}
