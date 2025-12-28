macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < I : DoubleEndedFallibleIterator + ? Sized > DoubleEndedFallibleIterator for & mut I { # [inline] fn next_back (& mut self) -> Result < Option < I :: Item > , I :: Error > { (* * self) . next_back () } }
    };
}

impl_49!()