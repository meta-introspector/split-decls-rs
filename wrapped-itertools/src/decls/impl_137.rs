macro_rules! deps {
    () => {
        Positions!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < I , F > DoubleEndedIterator for Positions < I , F > where I : DoubleEndedIterator + ExactSizeIterator , F : FnMut (I :: Item) -> bool , { fn next_back (& mut self) -> Option < Self :: Item > { let f = & mut self . f ; self . iter . by_ref () . rev () . find_map (| (count , val) | f (val) . then_some (count)) } fn rfold < B , G > (self , init : B , mut func : G) -> B where G : FnMut (B , Self :: Item) -> B , { let mut f = self . f ; self . iter . rfold (init , | mut acc , (count , val) | { if f (val) { acc = func (acc , count) ; } acc }) } }
    };
}

impl_137!();