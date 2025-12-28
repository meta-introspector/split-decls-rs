macro_rules! deps {
    () => {
        Iterator!();
        FallibleIterator!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < I > iter :: Iterator for Iterator < I > where I : FallibleIterator , { type Item = Result < I :: Item , I :: Error > ; # [inline] fn next (& mut self) -> Option < Result < I :: Item , I :: Error > > { match self . 0 . next () { Ok (Some (v)) => Some (Ok (v)) , Ok (None) => None , Err (e) => Some (Err (e)) , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
    };
}

impl_96!();