macro_rules! deps {
    () => {
        Memchr2!();
        Iter!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < 'h > Memchr2 < 'h > { # [doc = " Returns an iterator over all occurrences of the needle bytes in the"] # [doc = " given haystack."] # [doc = ""] # [doc = " The iterator returned implements `DoubleEndedIterator`. This means it"] # [doc = " can also be used to find occurrences in reverse order."] # [inline] pub fn new (needle1 : u8 , needle2 : u8 , haystack : & 'h [u8]) -> Memchr2 < 'h > { Memchr2 { needle1 , needle2 , it : crate :: arch :: generic :: memchr :: Iter :: new (haystack) , } } }
    };
}

impl_308!();