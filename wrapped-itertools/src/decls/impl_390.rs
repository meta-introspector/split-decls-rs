macro_rules! deps {
    () => {
        PadUsing!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        impl < I , F > FusedIterator for PadUsing < I , F > where I : FusedIterator , F : FnMut (usize) -> I :: Item , { }
    };
}

impl_390!()