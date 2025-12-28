macro_rules! deps {
    () => {
        Union!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < 'a > Iterator for Union < 'a > { type Item = usize ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_104!()