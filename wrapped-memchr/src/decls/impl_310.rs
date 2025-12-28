macro_rules! deps {
    () => {
        Memchr2!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < 'h > DoubleEndedIterator for Memchr2 < 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | { memrchr2_raw (self . needle1 , self . needle2 , s , e) }) } } }
    };
}

impl_310!();