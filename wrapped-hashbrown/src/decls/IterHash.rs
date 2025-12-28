macro_rules! deps {
    () => {
        RawIterHash!();
        HashTable!();
    };
}

macro_rules! IterHash {
    () => {
        deps!();
        # [doc = " An iterator over the entries of a `HashTable` that could match a given hash."] # [doc = " The iterator element type is `&'a T`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter_hash`] method on [`HashTable`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter_hash`]: struct.HashTable.html#method.iter_hash"] # [doc = " [`HashTable`]: struct.HashTable.html"] pub struct IterHash < 'a , T > { inner : RawIterHash < T > , marker : PhantomData < & 'a T > , }
    };
}

IterHash!();