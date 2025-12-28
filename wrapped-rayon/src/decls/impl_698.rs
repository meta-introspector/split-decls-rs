macro_rules! deps {
    () => {
        MapWithIter!();
    };
}

macro_rules! impl_698 {
    () => {
        deps!();
        impl < 'f , I , U , F , R > ExactSizeIterator for MapWithIter < 'f , I , U , F > where I : ExactSizeIterator , F : Fn (& mut U , I :: Item) -> R + Sync , R : Send , { }
    };
}

impl_698!()