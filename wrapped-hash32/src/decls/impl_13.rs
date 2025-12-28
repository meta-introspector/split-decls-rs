macro_rules! deps {
    () => {
        Index!();
        Murmur3Hasher!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Murmur3Hasher { # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `self.index.usize() + buf.len() <= 4`."] unsafe fn push (& mut self , buf : & [u8]) { let start = self . index . usize () ; let len = buf . len () ; for i in 0 .. len { unsafe { * self . buf . bytes . assume_init_mut () . get_unchecked_mut (start + i) = * buf . get_unchecked (i) ; } } self . index = Index :: from (start + len) ; } }
    };
}

impl_13!()