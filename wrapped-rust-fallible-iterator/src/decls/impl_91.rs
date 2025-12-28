macro_rules! deps {
    () => {
        Empty!();
        FallibleIterator!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < T , E > FallibleIterator for Empty < T , E > { type Item = T ; type Error = E ; # [inline] fn next (& mut self) -> Result < Option < T > , E > { Ok (None) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
    };
}

impl_91!()