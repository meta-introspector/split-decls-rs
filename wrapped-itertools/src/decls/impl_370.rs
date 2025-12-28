macro_rules! deps {
    () => {
        MultiPeek!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        impl < I > MultiPeek < I > where I : Iterator , { # [doc = " Reset the peeking “cursor”"] pub fn reset_peek (& mut self) { self . index = 0 ; } }
    };
}

impl_370!()