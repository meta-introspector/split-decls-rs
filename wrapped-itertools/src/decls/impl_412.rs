macro_rules! deps {
    () => {
        PeekingNext!();
        PeekingTakeWhile!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        impl < I , F > Iterator for PeekingTakeWhile < '_ , I , F > where I : PeekingNext , F : FnMut (& I :: Item) -> bool , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . iter . peeking_next (& mut self . f) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . iter . size_hint () . 1) } }
    };
}

impl_412!()