macro_rules! deps {
    () => {
        RChunksExactMut!();
    };
}

macro_rules! impl_1191 {
    () => {
        deps!();
        impl < 'data , T : Send > RChunksExactMut < 'data , T > { pub (super) fn new (chunk_size : usize , slice : & 'data mut [T]) -> Self { let rem_len = slice . len () % chunk_size ; let (rem , slice) = slice . split_at_mut (rem_len) ; Self { chunk_size , slice , rem , } } # [doc = " Return the remainder of the original slice that is not going to be"] # [doc = " returned by the iterator. The returned slice has at most `chunk_size-1`"] # [doc = " elements."] # [doc = ""] # [doc = " Note that this has to consume `self` to return the original lifetime of"] # [doc = " the data, which prevents this from actually being used as a parallel"] # [doc = " iterator since that also consumes. This method is provided for parity"] # [doc = " with `std::iter::RChunksExactMut`, but consider calling `remainder()` or"] # [doc = " `take_remainder()` as alternatives."] pub fn into_remainder (self) -> & 'data mut [T] { self . rem } # [doc = " Return the remainder of the original slice that is not going to be"] # [doc = " returned by the iterator. The returned slice has at most `chunk_size-1`"] # [doc = " elements."] # [doc = ""] # [doc = " Consider `take_remainder()` if you need access to the data with its"] # [doc = " original lifetime, rather than borrowing through `&mut self` here."] pub fn remainder (& mut self) -> & mut [T] { self . rem } # [doc = " Return the remainder of the original slice that is not going to be"] # [doc = " returned by the iterator. The returned slice has at most `chunk_size-1`"] # [doc = " elements. Subsequent calls will return an empty slice."] pub fn take_remainder (& mut self) -> & 'data mut [T] { std :: mem :: take (& mut self . rem) } }
    };
}

impl_1191!()