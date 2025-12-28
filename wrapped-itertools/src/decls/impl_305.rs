macro_rules! deps {
    () => {
        IntersperseWith!();
        IntersperseElement!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl < I , ElemF > FusedIterator for IntersperseWith < I , ElemF > where I : Iterator , ElemF : IntersperseElement < I :: Item > , { }
    };
}

impl_305!()