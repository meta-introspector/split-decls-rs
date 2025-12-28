macro_rules! RChunksExactMut {
    () => {
        # [doc = " Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end."] # [derive (Debug)] pub struct RChunksExactMut < 'data , T : Send > { chunk_size : usize , slice : & 'data mut [T] , rem : & 'data mut [T] , }
    };
}

RChunksExactMut!();