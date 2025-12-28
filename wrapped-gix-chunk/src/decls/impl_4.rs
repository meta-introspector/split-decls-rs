macro_rules! deps {
    () => {
        Entry!();
        Index!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl file :: Index { # [doc = " Provided a mapped file at the beginning via `data`, starting at `toc_offset` decode all chunk information to return"] # [doc = " an index with `num_chunks` chunks."] pub fn from_bytes (data : & [u8] , toc_offset : usize , num_chunks : u32) -> Result < Self , Error > { if num_chunks == 0 { return Err (Error :: Empty) ; } let data_len : u64 = data . len () as u64 ; let mut chunks = Vec :: with_capacity (num_chunks as usize) ; let mut toc_entry = & data [toc_offset ..] ; let expected_min_size = (num_chunks as usize + 1) * file :: Index :: ENTRY_SIZE ; if toc_entry . len () < expected_min_size { return Err (Error :: TocTooSmall { expected : expected_min_size , actual : toc_entry . len () , }) ; } for _ in 0 .. num_chunks { let (kind , offset) = toc_entry . split_at (4) ; let kind = to_kind (kind) ; if kind == crate :: SENTINEL { return Err (Error :: EarlySentinelValue) ; } if chunks . iter () . any (| c : & index :: Entry | c . kind == kind) { return Err (Error :: DuplicateChunk { kind }) ; } let offset = be_u64 (offset) ; if offset > data_len { return Err (Error :: ChunkSizeOutOfBounds { offset , file_length : data_len , }) ; } toc_entry = & toc_entry [file :: Index :: ENTRY_SIZE ..] ; let next_offset = be_u64 (& toc_entry [4 ..]) ; if next_offset > data_len { return Err (Error :: ChunkSizeOutOfBounds { offset : next_offset , file_length : data_len , }) ; } if next_offset <= offset { return Err (Error :: NonIncrementalChunkOffsets) ; } chunks . push (index :: Entry { kind , offset : Range { start : offset , end : next_offset , } , }) ; } let sentinel = to_kind (& toc_entry [.. 4]) ; if sentinel != crate :: SENTINEL { return Err (Error :: MissingSentinelValue { actual : sentinel }) ; } Ok (file :: Index { chunks , will_write : false , }) } }
    };
}

impl_4!()