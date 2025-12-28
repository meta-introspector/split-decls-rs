macro_rules! deps {
    () => {
        RcIter!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl < A , I > Iterator for RcIter < I > where I : Iterator < Item = A > , { type Item = A ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . rciter . borrow_mut () . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . rciter . borrow () . size_hint () . 1) } }
    };
}

impl_459!();