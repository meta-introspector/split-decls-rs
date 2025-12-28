macro_rules! deps {
    () => {
        InterleaveSeq!();
    };
}

macro_rules! impl_635 {
    () => {
        deps!();
        impl < I , J > DoubleEndedIterator for InterleaveSeq < I , J > where I : DoubleEndedIterator + ExactSizeIterator , J : DoubleEndedIterator < Item = I :: Item > + ExactSizeIterator < Item = I :: Item > , { # [inline] fn next_back (& mut self) -> Option < I :: Item > { match self . i . len () . cmp (& self . j . len ()) { Ordering :: Less => self . j . next_back () , Ordering :: Equal => { if self . i_next { self . i . next_back () } else { self . j . next_back () } } Ordering :: Greater => self . i . next_back () , } } }
    };
}

impl_635!();