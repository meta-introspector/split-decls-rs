macro_rules! Chunks {
    () => {
        # [doc = " Parallel iterator over immutable non-overlapping chunks of a slice"] # [derive (Debug)] pub struct Chunks < 'data , T > { chunk_size : usize , slice : & 'data [T] , }
    };
}

Chunks!()