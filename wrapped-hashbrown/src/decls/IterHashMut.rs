macro_rules! deps {
    () => {
        HashTable!();
        RawIterHash!();
    };
}

macro_rules! IterHashMut {
    () => {
        deps!();
        # [doc = " A mutable iterator over the entries of a `HashTable` that could match a given hash."] # [doc = " The iterator element type is `&'a mut T`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter_hash_mut`] method on [`HashTable`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter_hash_mut`]: struct.HashTable.html#method.iter_hash_mut"] # [doc = " [`HashTable`]: struct.HashTable.html"] pub struct IterHashMut < 'a , T > { inner : RawIterHash < T > , marker : PhantomData < & 'a mut T > , }
    };
}

IterHashMut!()