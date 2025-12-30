// Generated macro for impl_108 (impl)
macro_rules! Depcrate_collections_vecimpl_108 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_108"}
// Dependencies: {}
# [doc = " Extend implementation that copies elements out of references before pushing them onto the Vec."] # [doc = ""] # [doc = " This implementation is specialized for slice iterators, where it uses [`copy_from_slice`] to"] # [doc = " append the entire slice at once."] # [doc = ""] # [doc = " [`copy_from_slice`]: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice"] impl < 'a , 'bump , T : 'a + Copy > Extend < & 'a T > for Vec < 'bump , T > { fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) } }
};
}
