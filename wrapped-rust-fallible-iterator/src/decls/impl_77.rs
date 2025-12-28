macro_rules! deps {
    () => {
        Take!();
        FallibleIterator!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < I > FallibleIterator for Take < I > where I : FallibleIterator , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { if self . remaining == 0 { return Ok (None) ; } let next = self . it . next () ; if let Ok (Some (_)) = next { self . remaining -= 1 ; } next } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let hint = self . it . size_hint () ; (cmp :: min (hint . 0 , self . remaining) , hint . 1 . map (| n | cmp :: min (n , self . remaining)) ,) } }
    };
}

impl_77!()