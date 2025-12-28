macro_rules! deps {
    () => {
        FallibleIterator!();
        Empty!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < T , E > FallibleIterator for Empty < T , E > { type Item = T ; type Error = E ; # [inline] fn next (& mut self) -> Result < Option < T > , E > { Ok (None) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
    };
}

impl_132!()