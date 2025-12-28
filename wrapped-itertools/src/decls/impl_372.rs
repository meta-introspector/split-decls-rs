macro_rules! deps {
    () => {
        PeekingNext!();
        MultiPeek!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        impl < I > PeekingNext for MultiPeek < I > where I : Iterator , { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { if self . buf . is_empty () { if let Some (r) = self . peek () { if ! accept (r) { return None ; } } } else if let Some (r) = self . buf . front () { if ! accept (r) { return None ; } } self . next () } }
    };
}

impl_372!();