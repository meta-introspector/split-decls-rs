macro_rules! deps {
    () => {
        ChunksExact!();
    };
}

macro_rules! impl_1151 {
    () => {
        deps!();
        impl < 'data , T > ChunksExact < 'data , T > { pub (super) fn new (chunk_size : usize , slice : & 'data [T]) -> Self { let rem_len = slice . len () % chunk_size ; let len = slice . len () - rem_len ; let (slice , rem) = slice . split_at (len) ; Self { chunk_size , slice , rem , } } # [doc = " Return the remainder of the original slice that is not going to be"] # [doc = " returned by the iterator. The returned slice has at most `chunk_size-1`"] # [doc = " elements."] pub fn remainder (& self) -> & 'data [T] { self . rem } }
    };
}

impl_1151!()