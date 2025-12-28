macro_rules! deps {
    () => {
        HashTable!();
        RawParIter!();
    };
}

macro_rules! ParIter {
    () => {
        deps!();
        # [doc = " Parallel iterator over shared references to entries in a map."] # [doc = ""] # [doc = " This iterator is created by the [`par_iter`] method on [`HashTable`]"] # [doc = " (provided by the [`IntoParallelRefIterator`] trait)."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_iter`]: /hashbrown/struct.HashTable.html#method.par_iter"] # [doc = " [`HashTable`]: /hashbrown/struct.HashTable.html"] # [doc = " [`IntoParallelRefIterator`]: https://docs.rs/rayon/1.0/rayon/iter/trait.IntoParallelRefIterator.html"] pub struct ParIter < 'a , T > { inner : RawParIter < T > , marker : PhantomData < & 'a T > , }
    };
}

ParIter!()