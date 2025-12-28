macro_rules! deps {
    () => {
        UniqueBy!();
    };
}

macro_rules! impl_534 {
    () => {
        deps!();
        impl < I , V , F > DoubleEndedIterator for UniqueBy < I , V , F > where I : DoubleEndedIterator , V : Eq + Hash , F : FnMut (& I :: Item) -> V , { fn next_back (& mut self) -> Option < Self :: Item > { let Self { iter , used , f } = self ; iter . rfind (| v | used . insert (f (v) , ()) . is_none ()) } }
    };
}

impl_534!()