macro_rules! deps {
    () => {
        RawParDrain!();
        HashTable!();
    };
}

macro_rules! ParDrain {
    () => {
        deps!();
        # [doc = " Parallel draining iterator over entries of a map."] # [doc = ""] # [doc = " This iterator is created by the [`par_drain`] method on [`HashTable`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_drain`]: /hashbrown/struct.HashTable.html#method.par_drain"] # [doc = " [`HashTable`]: /hashbrown/struct.HashTable.html"] pub struct ParDrain < 'a , T , A : Allocator = Global > { inner : RawParDrain < 'a , T , A > , }
    };
}

ParDrain!()