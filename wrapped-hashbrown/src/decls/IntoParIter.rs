macro_rules! deps {
    () => {
        HashTable!();
        RawIntoParIter!();
    };
}

macro_rules! IntoParIter {
    () => {
        deps!();
        # [doc = " Parallel iterator over entries of a consumed map."] # [doc = ""] # [doc = " This iterator is created by the [`into_par_iter`] method on [`HashTable`]"] # [doc = " (provided by the [`IntoParallelIterator`] trait)."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`into_par_iter`]: /hashbrown/struct.HashTable.html#method.into_par_iter"] # [doc = " [`HashTable`]: /hashbrown/struct.HashTable.html"] # [doc = " [`IntoParallelIterator`]: https://docs.rs/rayon/1.0/rayon/iter/trait.IntoParallelIterator.html"] pub struct IntoParIter < T , A : Allocator = Global > { inner : RawIntoParIter < T , A > , }
    };
}

IntoParIter!()