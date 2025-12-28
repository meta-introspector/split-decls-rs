macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < 'a , T : 'a , const CAP : usize > DoubleEndedIterator for Drain < 'a , T , CAP > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } }
    };
}

impl_62!()