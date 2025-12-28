macro_rules! deps {
    () => {
        ChunkBy!();
        GroupInner!();
        ChunkIndex!();
        Chunk!();
    };
}

macro_rules! IntoChunks {
    () => {
        deps!();
        # [doc = " `ChunkLazy` is the storage for a lazy chunking operation."] # [doc = ""] # [doc = " `IntoChunks` behaves just like `ChunkBy`: it is iterable, and"] # [doc = " it only buffers if several chunk iterators are alive at the same time."] # [doc = ""] # [doc = " This type implements [`IntoIterator`] (it is **not** an iterator"] # [doc = " itself), because the chunk iterators need to borrow from this"] # [doc = " value. It should be stored in a local variable or temporary and"] # [doc = " iterated."] # [doc = ""] # [doc = " Iterator element type is `Chunk`, each chunk's iterator."] # [doc = ""] # [doc = " See [`.chunks()`](crate::Itertools::chunks) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct IntoChunks < I > where I : Iterator , { inner : RefCell < GroupInner < usize , I , ChunkIndex > > , index : Cell < usize > , }
    };
}

IntoChunks!()