macro_rules! ChunksMut {
    () => {
        # [doc = " Parallel iterator over mutable non-overlapping chunks of a slice"] # [derive (Debug)] pub struct ChunksMut < 'data , T > { chunk_size : usize , slice : & 'data mut [T] , }
    };
}

ChunksMut!();