// Generated macro for IterMut (struct)
macro_rules! Depcrate_tableIterMut {
() => {
// Module: crate::table
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the entries of a `HashTable` in arbitrary order."] # [doc = " The iterator element type is `&'a mut T`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter_mut`] method on [`HashTable`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter_mut`]: struct.HashTable.html#method.iter_mut"] # [doc = " [`HashTable`]: struct.HashTable.html"] pub struct IterMut < 'a , T > { inner : RawIter < T > , marker : PhantomData < & 'a mut T > , }
};
}
