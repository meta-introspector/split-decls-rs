macro_rules! deps {
    () => {
        InterleaveShortest!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < I , J > FusedIterator for InterleaveShortest < I , J > where I : FusedIterator , J : FusedIterator < Item = I :: Item > , { }
    };
}

impl_77!()