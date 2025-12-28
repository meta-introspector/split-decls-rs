macro_rules! deps {
    () => {
        Memchr3!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < 'h > DoubleEndedIterator for Memchr3 < 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | { memrchr3_raw (self . needle1 , self . needle2 , self . needle3 , s , e) }) } } }
    };
}

impl_315!()