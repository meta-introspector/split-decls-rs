macro_rules! deps {
    () => {
        Memchr!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl < 'h > DoubleEndedIterator for Memchr < 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | memrchr_raw (self . needle1 , s , e)) } } }
    };
}

impl_305!();