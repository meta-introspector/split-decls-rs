macro_rules! deps {
    () => {
        LenType!();
        IntoIter!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize > Iterator for IntoIter < T , N , LenT > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if self . next < self . vec . len { let item = unsafe { self . vec . buffer . buffer . get_unchecked_mut (self . next . into_usize ()) . as_ptr () . read () } ; self . next += LenT :: one () ; Some (item) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
    };
}

impl_304!()