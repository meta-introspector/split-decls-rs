macro_rules! deps {
    () => {
        Memchr!();
    };
}

macro_rules! memrchr_iter {
    () => {
        deps!();
        # [doc = " Returns an iterator over all occurrences of the needle in a haystack, in"] # [doc = " reverse."] # [inline] pub fn memrchr_iter (needle : u8 , haystack : & [u8]) -> Rev < Memchr < '_ > > { Memchr :: new (needle , haystack) . rev () }
    };
}

memrchr_iter!();