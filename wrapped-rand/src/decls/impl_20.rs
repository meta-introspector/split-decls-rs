macro_rules! deps {
    () => {
        Iter!();
        Distribution!();
        Rng!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < D , R , T > Iterator for Iter < D , R , T > where D : Distribution < T > , R : Rng , { type Item = T ; # [inline (always)] fn next (& mut self) -> Option < T > { Some (self . distr . sample (& mut self . rng)) } fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
    };
}

impl_20!()