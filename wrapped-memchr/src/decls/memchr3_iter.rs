macro_rules! deps {
    () => {
        Memchr3!();
    };
}

macro_rules! memchr3_iter {
    () => {
        deps!();
        # [doc = " Returns an iterator over all occurrences of the needles in a haystack."] # [doc = ""] # [doc = " The iterator returned implements `DoubleEndedIterator`. This means it"] # [doc = " can also be used to find occurrences in reverse order."] # [inline] pub fn memchr3_iter < 'h > (needle1 : u8 , needle2 : u8 , needle3 : u8 , haystack : & 'h [u8] ,) -> Memchr3 < 'h > { Memchr3 :: new (needle1 , needle2 , needle3 , haystack) }
    };
}

memchr3_iter!()