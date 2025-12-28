macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < I , F > DoubleEndedIterator for Update < I , F > where I : DoubleEndedIterator , F : FnMut (& mut I :: Item) , { fn next_back (& mut self) -> Option < Self :: Item > { if let Some (mut v) = self . iter . next_back () { (self . f) (& mut v) ; Some (v) } else { None } } }
    };
}

impl_144!()