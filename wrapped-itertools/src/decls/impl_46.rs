macro_rules! deps {
    () => {
        MapSpecialCaseFn!();
        MapSpecialCase!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < I , R > DoubleEndedIterator for MapSpecialCase < I , R > where I : DoubleEndedIterator , R : MapSpecialCaseFn < I :: Item > , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| i | self . f . call (i)) } }
    };
}

impl_46!();