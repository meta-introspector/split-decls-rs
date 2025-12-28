macro_rules! deps {
    () => {
        Take!();
        FallibleStreamingIterator!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < I > FallibleStreamingIterator for Take < I > where I : FallibleStreamingIterator , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { if self . n != 0 { self . it . advance () ? ; self . n -= 1 ; } else { self . done = true ; } Ok (()) } # [inline] fn get (& self) -> Option < & I :: Item > { if self . done { None } else { self . it . get () } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let (lower , upper) = self . it . size_hint () ; let lower = cmp :: min (lower , self . n) ; let upper = match upper { Some (x) if x < self . n => Some (x) , _ => Some (self . n) } ; (lower , upper) } }
    };
}

impl_32!()