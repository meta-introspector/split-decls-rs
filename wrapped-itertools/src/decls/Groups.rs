macro_rules! deps {
    () => {
        ChunkBy!();
        Group!();
    };
}

macro_rules! Groups {
    () => {
        deps!();
        # [doc = " An iterator that yields the Group iterators."] # [doc = ""] # [doc = " Iterator element type is `(K, Group)`:"] # [doc = " the group's key `K` and the group's iterator."] # [doc = ""] # [doc = " See [`.chunk_by()`](crate::Itertools::chunk_by) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Groups < 'a , K , I , F > where I : Iterator + 'a , I :: Item : 'a , K : 'a , F : 'a , { parent : & 'a ChunkBy < K , I , F > , }
    };
}

Groups!();