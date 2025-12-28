macro_rules! deps {
    () => {
        PutBack!();
        PeekingNext!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        impl < I > PeekingNext for PutBack < I > where I : Iterator , { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { if let Some (r) = self . next () { if ! accept (& r) { self . put_back (r) ; return None ; } Some (r) } else { None } } }
    };
}

impl_402!();