macro_rules! deps {
    () => {
        FallibleIterator!();
        Repeat!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < T : Clone , E > FallibleIterator for Repeat < T , E > { type Item = T ; type Error = E ; # [inline] fn next (& mut self) -> Result < Option < Self :: Item > , Self :: Error > { Ok (Some (self . 0 . clone ())) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (usize :: max_value () , None) } }
    };
}

impl_142!()