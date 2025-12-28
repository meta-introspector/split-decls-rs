macro_rules! deps {
    () => {
        RepeatErr!();
        FallibleIterator!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < T , E : Clone > FallibleIterator for RepeatErr < T , E > { type Item = T ; type Error = E ; # [inline] fn next (& mut self) -> Result < Option < Self :: Item > , Self :: Error > { Err (self . 1 . clone ()) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
    };
}

impl_104!()