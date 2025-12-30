// Generated macro for IntoIter (struct)
macro_rules! Depcrate_setIntoIter {
() => {
// Module: crate::set
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the items of a `HashSet`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`HashSet`]"] # [doc = " (provided by the `IntoIterator` trait). See its documentation for more."] # [doc = ""] # [doc = " [`HashSet`]: struct.HashSet.html"] # [doc = " [`into_iter`]: struct.HashSet.html#method.into_iter"] pub struct IntoIter < K , A : Allocator = Global > { iter : map :: IntoIter < K , () , A > , }
};
}
