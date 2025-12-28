macro_rules! deps {
    () => {
        PeekingNext!();
        PeekNth!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl < I > PeekingNext for PeekNth < I > where I : Iterator , { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { self . peek () . filter (| item | accept (item)) ? ; self . next () } }
    };
}

impl_397!()