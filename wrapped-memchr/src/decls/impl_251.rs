macro_rules! deps {
    () => {
        TwoIter!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl < 'a , 'h > DoubleEndedIterator for TwoIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
    };
}

impl_251!()