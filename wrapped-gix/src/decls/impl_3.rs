macro_rules! deps {
    () => {
        Iter!();
        Item!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < I , EFN , E > Iterator for Iter < I , EFN > where I : Iterator , EFN : FnOnce () -> E , { type Item = Result < I :: Item , E > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } }
    };
}

impl_3!()