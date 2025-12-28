macro_rules! deps {
    () => {
        TakeWhileInclusive!();
    };
}

macro_rules! impl_494 {
    () => {
        deps!();
        impl < I , F > FusedIterator for TakeWhileInclusive < I , F > where I : Iterator , F : FnMut (& I :: Item) -> bool , { }
    };
}

impl_494!()