macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < 'a , T : 'a , const CAP : usize > Iterator for Drain < 'a , T , CAP > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_61!()