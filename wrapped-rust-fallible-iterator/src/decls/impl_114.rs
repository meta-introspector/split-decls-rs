macro_rules! deps {
    () => {
        SkipWhile!();
        FallibleIterator!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < I , P > FallibleIterator for SkipWhile < I , P > where I : FallibleIterator , P : FnMut (& I :: Item) -> Result < bool , I :: Error > , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { let flag = & mut self . flag ; let pred = & mut self . predicate ; self . it . find (move | x | { if * flag || ! pred (x) ? { * flag = true ; Ok (true) } else { Ok (false) } }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let hint = self . it . size_hint () ; if self . flag { hint } else { (0 , hint . 1) } } }
    };
}

impl_114!()