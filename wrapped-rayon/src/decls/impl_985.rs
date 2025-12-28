macro_rules! deps {
    () => {
        UpdateSeq!();
    };
}

macro_rules! impl_985 {
    () => {
        deps!();
        impl < I , F > DoubleEndedIterator for UpdateSeq < I , F > where I : DoubleEndedIterator , F : Fn (& mut I :: Item) , { fn next_back (& mut self) -> Option < Self :: Item > { let mut v = self . base . next_back () ? ; (self . update_op) (& mut v) ; Some (v) } }
    };
}

impl_985!()