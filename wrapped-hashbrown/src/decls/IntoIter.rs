macro_rules! deps {
    () => {
        HashTable!();
        RawIntoIter!();
    };
}

macro_rules! IntoIter {
    () => {
        deps!();
        # [doc = " An owning iterator over the entries of a `HashTable` in arbitrary order."] # [doc = " The iterator element type is `T`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`HashTable`]"] # [doc = " (provided by the [`IntoIterator`] trait). See its documentation for more."] # [doc = " The table cannot be used after calling that method."] # [doc = ""] # [doc = " [`into_iter`]: struct.HashTable.html#method.into_iter"] # [doc = " [`HashTable`]: struct.HashTable.html"] # [doc = " [`IntoIterator`]: https://doc.rust-lang.org/core/iter/trait.IntoIterator.html"] pub struct IntoIter < T , A = Global > where A : Allocator , { inner : RawIntoIter < T , A > , }
    };
}

IntoIter!()