macro_rules! deps {
    () => {
        UniqueBy!();
    };
}

macro_rules! impl_535 {
    () => {
        deps!();
        impl < I , V , F > FusedIterator for UniqueBy < I , V , F > where I : FusedIterator , V : Eq + Hash , F : FnMut (& I :: Item) -> V , { }
    };
}

impl_535!()