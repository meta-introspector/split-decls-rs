macro_rules! deps {
    () => {
        PeekingNext!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        impl < I > PeekingNext for Peekable < I > where I : Iterator , { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { if let Some (r) = self . peek () { if ! accept (r) { return None ; } } self . next () } }
    };
}

impl_401!()