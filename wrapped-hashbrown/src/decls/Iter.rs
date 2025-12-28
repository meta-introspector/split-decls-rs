macro_rules! deps {
    () => {
        RawIter!();
        HashTable!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " An iterator over the entries of a `HashTable` in arbitrary order."] # [doc = " The iterator element type is `&'a T`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`] method on [`HashTable`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter`]: struct.HashTable.html#method.iter"] # [doc = " [`HashTable`]: struct.HashTable.html"] pub struct Iter < 'a , T > { inner : RawIter < T > , marker : PhantomData < & 'a T > , }
    };
}

Iter!();