macro_rules! RChunksExact {
    () => {
        # [doc = " Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end."] # [derive (Debug)] pub struct RChunksExact < 'data , T > { chunk_size : usize , slice : & 'data [T] , rem : & 'data [T] , }
    };
}

RChunksExact!();