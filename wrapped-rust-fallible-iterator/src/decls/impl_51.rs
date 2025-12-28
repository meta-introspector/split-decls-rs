macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < I : DoubleEndedFallibleIterator + ? Sized > DoubleEndedFallibleIterator for Box < I > { # [inline] fn next_back (& mut self) -> Result < Option < I :: Item > , I :: Error > { (* * self) . next_back () } }
    };
}

impl_51!()