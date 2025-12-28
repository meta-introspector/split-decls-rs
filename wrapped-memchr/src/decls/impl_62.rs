macro_rules! deps {
    () => {
        OneIter!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < 'a , 'h > DoubleEndedIterator for OneIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
    };
}

impl_62!()