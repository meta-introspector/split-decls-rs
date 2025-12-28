macro_rules! deps {
    () => {
        Memchr3!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < 'h > Iterator for Memchr3 < 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | { memchr3_raw (self . needle1 , self . needle2 , self . needle3 , s , e) }) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_314!()