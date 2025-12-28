macro_rules! deps {
    () => {
        Memchr!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl < 'h > Iterator for Memchr < 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | memchr_raw (self . needle1 , s , e)) } } # [inline] fn count (self) -> usize { self . it . count (| s , e | { unsafe { count_raw (self . needle1 , s , e) } }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_304!()