macro_rules! RChunks {
    () => {
        # [doc = " Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end."] # [derive (Debug)] pub struct RChunks < 'data , T > { chunk_size : usize , slice : & 'data [T] , }
    };
}

RChunks!()