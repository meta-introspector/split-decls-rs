macro_rules! deps {
    () => {
        RChunksExact!();
    };
}

macro_rules! impl_1178 {
    () => {
        deps!();
        impl < 'data , T > RChunksExact < 'data , T > { pub (super) fn new (chunk_size : usize , slice : & 'data [T]) -> Self { let rem_len = slice . len () % chunk_size ; let (rem , slice) = slice . split_at (rem_len) ; Self { chunk_size , slice , rem , } } # [doc = " Return the remainder of the original slice that is not going to be"] # [doc = " returned by the iterator. The returned slice has at most `chunk_size-1`"] # [doc = " elements."] pub fn remainder (& self) -> & 'data [T] { self . rem } }
    };
}

impl_1178!();