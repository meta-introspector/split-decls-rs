macro_rules! deps {
    () => {
        ThreeIter!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < 'a , 'h > DoubleEndedIterator for ThreeIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
    };
}

impl_257!();