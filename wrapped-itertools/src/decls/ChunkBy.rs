macro_rules! deps {
    () => {
        GroupInner!();
    };
}

macro_rules! ChunkBy {
    () => {
        deps!();
        # [doc = " `ChunkBy` is the storage for the lazy grouping operation."] # [doc = ""] # [doc = " If the groups are consumed in their original order, or if each"] # [doc = " group is dropped without keeping it around, then `ChunkBy` uses"] # [doc = " no allocations. It needs allocations only if several group iterators"] # [doc = " are alive at the same time."] # [doc = ""] # [doc = " This type implements [`IntoIterator`] (it is **not** an iterator"] # [doc = " itself), because the group iterators need to borrow from this"] # [doc = " value. It should be stored in a local variable or temporary and"] # [doc = " iterated."] # [doc = ""] # [doc = " See [`.chunk_by()`](crate::Itertools::chunk_by) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct ChunkBy < K , I , F > where I : Iterator , { inner : RefCell < GroupInner < K , I , F > > , index : Cell < usize > , }
    };
}

ChunkBy!()