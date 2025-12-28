macro_rules! deps {
    () => {
        Allocator!();
        Drain!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < T , A : Allocator > DoubleEndedIterator for Drain < '_ , T , A > { # [inline (always)] fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } }
    };
}

impl_115!();