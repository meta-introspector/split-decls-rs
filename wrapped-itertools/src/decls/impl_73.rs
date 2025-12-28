macro_rules! deps {
    () => {
        Interleave!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < I , J > FusedIterator for Interleave < I , J > where I : Iterator , J : Iterator < Item = I :: Item > , { }
    };
}

impl_73!();