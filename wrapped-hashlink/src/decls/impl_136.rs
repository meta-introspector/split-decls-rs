macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < T : Hash + Eq + Clone , S : BuildHasher + Clone > Clone for LinkedHashSet < T , S > { # [inline] fn clone (& self) -> Self { let map = self . map . clone () ; Self { map } } }
    };
}

impl_136!();