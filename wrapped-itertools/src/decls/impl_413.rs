macro_rules! deps {
    () => {
        PeekingTakeWhile!();
        PeekingNext!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        impl < I , F > PeekingNext for PeekingTakeWhile < '_ , I , F > where I : PeekingNext , F : FnMut (& I :: Item) -> bool , { fn peeking_next < G > (& mut self , g : G) -> Option < Self :: Item > where G : FnOnce (& Self :: Item) -> bool , { let f = & mut self . f ; self . iter . peeking_next (| r | f (r) && g (r)) } }
    };
}

impl_413!();