// Generated macro for IntoIter (struct)
macro_rules! Depcrate_collections_vecIntoIter {
() => {
// Module: crate::collections::vec
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An iterator that moves out of a vector."] # [doc = ""] # [doc = " This `struct` is created by the [`Vec::into_iter`] method"] # [doc = " (provided by the [`IntoIterator`] trait)."] # [doc = ""] # [doc = " [`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html"] pub struct IntoIter < 'bump , T > { phantom : PhantomData < & 'bump [T] > , ptr : * const T , end : * const T , }
};
}
