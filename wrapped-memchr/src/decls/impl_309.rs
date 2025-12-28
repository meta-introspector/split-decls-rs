macro_rules! deps {
    () => {
        Memchr2!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < 'h > Iterator for Memchr2 < 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | memchr2_raw (self . needle1 , self . needle2 , s , e)) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_309!();