macro_rules! deps {
    () => {
        IntoChunks!();
        Chunk!();
    };
}

macro_rules! Chunks {
    () => {
        deps!();
        # [doc = " An iterator that yields the Chunk iterators."] # [doc = ""] # [doc = " Iterator element type is `Chunk`."] # [doc = ""] # [doc = " See [`.chunks()`](crate::Itertools::chunks) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct Chunks < 'a , I > where I : Iterator + 'a , I :: Item : 'a , { parent : & 'a IntoChunks < I > , }
    };
}

Chunks!();