macro_rules! deps {
    () => {
        HashTable!();
        RawParIter!();
    };
}

macro_rules! ParIterMut {
    () => {
        deps!();
        # [doc = " Parallel iterator over mutable references to entries in a map."] # [doc = ""] # [doc = " This iterator is created by the [`par_iter_mut`] method on [`HashTable`]"] # [doc = " (provided by the [`IntoParallelRefMutIterator`] trait)."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_iter_mut`]: /hashbrown/struct.HashTable.html#method.par_iter_mut"] # [doc = " [`HashTable`]: /hashbrown/struct.HashTable.html"] # [doc = " [`IntoParallelRefMutIterator`]: https://docs.rs/rayon/1.0/rayon/iter/trait.IntoParallelRefMutIterator.html"] pub struct ParIterMut < 'a , T > { inner : RawParIter < T > , marker : PhantomData < & 'a mut T > , }
    };
}

ParIterMut!();