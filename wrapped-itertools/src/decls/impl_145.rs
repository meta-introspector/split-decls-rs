macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < I , F > FusedIterator for Update < I , F > where I : FusedIterator , F : FnMut (& mut I :: Item) , { }
    };
}

impl_145!();