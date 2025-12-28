macro_rules! deps {
    () => {
        DoubleEndedFallibleStreamingIterator!();
        Map!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < I , F , B > DoubleEndedFallibleStreamingIterator for Map < I , F , B > where I : DoubleEndedFallibleStreamingIterator , F : FnMut (& I :: Item) -> B , { # [inline] fn advance_back (& mut self) -> Result < () , I :: Error > { self . value = self . it . next_back () ? . map (& mut self . f) ; Ok (()) } }
    };
}

impl_20!()