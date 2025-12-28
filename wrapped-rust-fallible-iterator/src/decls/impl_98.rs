macro_rules! deps {
    () => {
        OnceErr!();
        FallibleIterator!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < T , E > FallibleIterator for OnceErr < T , E > { type Item = T ; type Error = E ; # [inline] fn next (& mut self) -> Result < Option < Self :: Item > , Self :: Error > { match self . 1 . take () { Some (value) => Err (value) , None => Ok (None) , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
    };
}

impl_98!()