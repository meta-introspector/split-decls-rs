// Generated macro for Difference (struct)
macro_rules! Depcrate_setDifference {
() => {
// Module: crate::set
// Provides: {"Difference"}
// Dependencies: {}
# [doc = " A lazy iterator producing elements in the difference of `HashSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`difference`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`HashSet`]: struct.HashSet.html"] # [doc = " [`difference`]: struct.HashSet.html#method.difference"] pub struct Difference < 'a , T , S , A : Allocator = Global > { iter : Iter < 'a , T > , other : & 'a HashSet < T , S , A > , }
};
}
