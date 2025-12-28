macro_rules! deps {
    () => {
        Once!();
        FallibleIterator!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < T , E > FallibleIterator for Once < T , E > { type Item = T ; type Error = E ; # [inline] fn next (& mut self) -> Result < Option < Self :: Item > , Self :: Error > { Ok (self . 0 . take ()) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { match self . 0 { Some (_) => (1 , Some (1)) , None => (0 , Some (0)) , } } }
    };
}

impl_95!()