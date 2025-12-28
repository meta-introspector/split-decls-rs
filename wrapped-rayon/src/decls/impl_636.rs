macro_rules! deps {
    () => {
        InterleaveSeq!();
    };
}

macro_rules! impl_636 {
    () => {
        deps!();
        impl < I , J > ExactSizeIterator for InterleaveSeq < I , J > where I : ExactSizeIterator , J : ExactSizeIterator < Item = I :: Item > , { # [inline] fn len (& self) -> usize { self . i . len () + self . j . len () } }
    };
}

impl_636!();