macro_rules! deps {
    () => {
        Word!();
        ChunkSize!();
    };
}

macro_rules! Chunk {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] enum Chunk { # [doc = " A chunk that is all zeros; we don't represent the zeros explicitly."] Zeros , # [doc = " A chunk that is all ones; we don't represent the ones explicitly."] Ones , # [doc = " A chunk that has a mix of zeros and ones, which are represented"] # [doc = " explicitly and densely. It never has all zeros or all ones."] # [doc = ""] # [doc = " If this is the final chunk there may be excess, unused words. This"] # [doc = " turns out to be both simpler and have better performance than"] # [doc = " allocating the minimum number of words, largely because we avoid having"] # [doc = " to store the length, which would make this type larger. These excess"] # [doc = " words are always zero, as are any excess bits in the final in-use word."] # [doc = ""] # [doc = " The `ChunkSize` field is the count of 1s set in the chunk, and"] # [doc = " must satisfy `0 < count < chunk_domain_size`."] # [doc = ""] # [doc = " The words are within an `Rc` because it's surprisingly common to"] # [doc = " duplicate an entire chunk, e.g. in `ChunkedBitSet::clone_from()`, or"] # [doc = " when a `Mixed` chunk is union'd into a `Zeros` chunk. When we do need"] # [doc = " to modify a chunk we use `Rc::make_mut`."] Mixed (ChunkSize , Rc < [Word ; CHUNK_WORDS] >) , }
    };
}

Chunk!()