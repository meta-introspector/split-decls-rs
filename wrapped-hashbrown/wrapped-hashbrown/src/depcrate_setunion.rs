// Generated macro for Union (struct)
macro_rules! Depcrate_setUnion {
() => {
// Module: crate::set
// Provides: {"Union"}
// Dependencies: {}
# [doc = " A lazy iterator producing elements in the union of `HashSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`union`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`HashSet`]: struct.HashSet.html"] # [doc = " [`union`]: struct.HashSet.html#method.union"] pub struct Union < 'a , T , S , A : Allocator = Global > { iter : Chain < Iter < 'a , T > , Difference < 'a , T , S , A > > , }
};
}
