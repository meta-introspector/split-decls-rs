macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < 'a , T : 'a , const N : usize > DoubleEndedIterator for Drain < 'a , T , N > { # [inline] fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| reference | unsafe { core :: ptr :: read (reference) }) } }
    };
}

impl_88!();