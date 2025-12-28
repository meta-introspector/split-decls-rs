macro_rules! deps {
    () => {
        RawDrain!();
        HashTable!();
    };
}

macro_rules! Drain {
    () => {
        deps!();
        # [doc = " A draining iterator over the items of a `HashTable`."] # [doc = ""] # [doc = " This `struct` is created by the [`drain`] method on [`HashTable`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`HashTable`]: struct.HashTable.html"] # [doc = " [`drain`]: struct.HashTable.html#method.drain"] pub struct Drain < 'a , T , A : Allocator = Global > { inner : RawDrain < 'a , T , A > , }
    };
}

Drain!();