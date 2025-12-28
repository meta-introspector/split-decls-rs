macro_rules! deps {
    () => {
        SizeHint!();
        Tuple1Combination!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < I : Iterator > Iterator for Tuple1Combination < I > { type Item = (I :: Item ,) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| x | (x ,)) } fn size_hint (& self) -> SizeHint { self . iter . size_hint () } fn count (self) -> usize { self . iter . count () } fn fold < B , F > (self , init : B , f : F) -> B where F : FnMut (B , Self :: Item) -> B , { self . iter . map (| x | (x ,)) . fold (init , f) } }
    };
}

impl_104!();