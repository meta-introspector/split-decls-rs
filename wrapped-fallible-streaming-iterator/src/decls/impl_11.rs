macro_rules! deps {
    () => {
        Empty!();
        FallibleStreamingIterator!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T , E > FallibleStreamingIterator for Empty < T , E > { type Item = T ; type Error = E ; # [inline] fn advance (& mut self) -> Result < () , E > { Ok (()) } # [inline] fn get (& self) -> Option < & T > { None } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
    };
}

impl_11!();