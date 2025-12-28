macro_rules! deps {
    () => {
        PeekingNext!();
    };
}

macro_rules! impl_400 {
    () => {
        deps!();
        impl < I > PeekingNext for & mut I where I : PeekingNext , { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { (* self) . peeking_next (accept) } }
    };
}

impl_400!()