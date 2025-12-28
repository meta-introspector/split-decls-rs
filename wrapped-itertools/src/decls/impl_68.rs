macro_rules! deps {
    () => {
        MultiProduct!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < I > std :: iter :: FusedIterator for MultiProduct < I > where I : Iterator + Clone , I :: Item : Clone , { }
    };
}

impl_68!();