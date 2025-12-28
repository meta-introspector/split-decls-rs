macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < 'a , K > Iterator for Iter < 'a , K > { type Item = & 'a K ; # [inline] fn next (& mut self) -> Option < & 'a K > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_159!()