macro_rules! deps {
    () => {
        Positions!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < I , F > FusedIterator for Positions < I , F > where I : FusedIterator , F : FnMut (I :: Item) -> bool , { }
    };
}

impl_138!();