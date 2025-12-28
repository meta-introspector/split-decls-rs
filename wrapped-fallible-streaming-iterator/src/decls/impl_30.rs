macro_rules! deps {
    () => {
        FallibleStreamingIterator!();
        SkipWhile!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < I , F > FallibleStreamingIterator for SkipWhile < I , F > where I : FallibleStreamingIterator , F : FnMut (& I :: Item) -> bool , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { if ! self . done { self . done = true ; let f = & mut self . f ; self . it . find (| i | ! f (i)) . map (| _ | ()) } else { self . it . advance () } } # [inline] fn get (& self) -> Option < & I :: Item > { self . it . get () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let hint = self . it . size_hint () ; if self . done { hint } else { (0 , hint . 1) } } }
    };
}

impl_30!();