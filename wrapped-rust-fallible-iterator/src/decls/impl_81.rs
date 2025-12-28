macro_rules! deps {
    () => {
        FallibleIterator!();
        Cycle!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < I > FallibleIterator for Cycle < I > where I : FallibleIterator + Clone , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { match self . cur . next () ? { None => { self . cur = self . it . clone () ; self . cur . next () } Some (v) => Ok (Some (v)) , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (usize :: max_value () , None) } }
    };
}

impl_81!()