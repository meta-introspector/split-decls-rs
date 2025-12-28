macro_rules! deps {
    () => {
        Memchr2!();
    };
}

macro_rules! memchr2_iter {
    () => {
        deps!();
        # [doc = " Returns an iterator over all occurrences of the needles in a haystack."] # [doc = ""] # [doc = " The iterator returned implements `DoubleEndedIterator`. This means it"] # [doc = " can also be used to find occurrences in reverse order."] # [inline] pub fn memchr2_iter < 'h > (needle1 : u8 , needle2 : u8 , haystack : & 'h [u8] ,) -> Memchr2 < 'h > { Memchr2 :: new (needle1 , needle2 , haystack) }
    };
}

memchr2_iter!();