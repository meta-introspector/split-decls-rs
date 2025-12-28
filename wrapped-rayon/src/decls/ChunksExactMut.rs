macro_rules! ChunksExactMut {
    () => {
        # [doc = " Parallel iterator over mutable non-overlapping chunks of a slice"] # [derive (Debug)] pub struct ChunksExactMut < 'data , T > { chunk_size : usize , slice : & 'data mut [T] , rem : & 'data mut [T] , }
    };
}

ChunksExactMut!()