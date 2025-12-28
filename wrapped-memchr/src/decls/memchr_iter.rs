macro_rules! deps {
    () => {
        Memchr!();
    };
}

macro_rules! memchr_iter {
    () => {
        deps!();
        # [doc = " Returns an iterator over all occurrences of the needle in a haystack."] # [doc = ""] # [doc = " The iterator returned implements `DoubleEndedIterator`. This means it"] # [doc = " can also be used to find occurrences in reverse order."] # [inline] pub fn memchr_iter < 'h > (needle : u8 , haystack : & 'h [u8]) -> Memchr < 'h > { Memchr :: new (needle , haystack) }
    };
}

memchr_iter!()