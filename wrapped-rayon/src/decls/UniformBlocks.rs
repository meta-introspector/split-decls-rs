macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! UniformBlocks {
    () => {
        deps!();
        # [doc = " `UniformBlocks` is a parallel iterator that consumes itself as a sequence"] # [doc = " of parallel blocks of constant sizes."] # [doc = ""] # [doc = " This struct is created by the [`by_uniform_blocks()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`by_uniform_blocks()`]: IndexedParallelIterator::by_uniform_blocks()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct UniformBlocks < I > { base : I , block_size : usize , }
    };
}

UniformBlocks!();