macro_rules! deps {
    () => {
        Empty!();
        DoubleEndedFallibleStreamingIterator!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < T , E > DoubleEndedFallibleStreamingIterator for Empty < T , E > { # [inline] fn advance_back (& mut self) -> Result < () , E > { Ok (()) } }
    };
}

impl_12!();