macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'a > Iterator for Bytes < 'a > { type Item = u8 ; # [inline] fn next (& mut self) -> Option < u8 > { self . it . next () . copied () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_78!()