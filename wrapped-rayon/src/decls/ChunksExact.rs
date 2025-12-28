macro_rules! ChunksExact {
    () => {
        # [doc = " Parallel iterator over immutable non-overlapping chunks of a slice"] # [derive (Debug)] pub struct ChunksExact < 'data , T > { chunk_size : usize , slice : & 'data [T] , rem : & 'data [T] , }
    };
}

ChunksExact!();