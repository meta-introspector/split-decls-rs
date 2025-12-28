macro_rules! deps {
    () => {
        Memchr2!();
    };
}

macro_rules! memrchr2_iter {
    () => {
        deps!();
        # [doc = " Returns an iterator over all occurrences of the needles in a haystack, in"] # [doc = " reverse."] # [inline] pub fn memrchr2_iter (needle1 : u8 , needle2 : u8 , haystack : & [u8] ,) -> Rev < Memchr2 < '_ > > { Memchr2 :: new (needle1 , needle2 , haystack) . rev () }
    };
}

memrchr2_iter!()