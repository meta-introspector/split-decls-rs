macro_rules! deps {
    () => {
        Map!();
        FallibleStreamingIterator!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < I , F , B > FallibleStreamingIterator for Map < I , F , B > where I : FallibleStreamingIterator , F : FnMut (& I :: Item) -> B , { type Item = B ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { self . value = self . it . next () ? . map (& mut self . f) ; Ok (()) } # [inline] fn get (& self) -> Option < & B > { self . value . as_ref () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_19!()