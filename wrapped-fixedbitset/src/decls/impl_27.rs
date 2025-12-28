macro_rules! deps {
    () => {
        SymmetricDifference!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < 'a > Iterator for SymmetricDifference < 'a > { type Item = usize ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_27!()