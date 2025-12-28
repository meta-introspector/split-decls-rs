macro_rules! deps {
    () => {
        MatchesIter!();
        Input!();
    };
}

macro_rules! impl_667 {
    () => {
        deps!();
        impl < 'h , F > MatchesIter < 'h , F > { # [doc = " Returns the current `Input` used by this iterator."] # [doc = ""] # [doc = " The `Input` returned is generally equivalent to the one used to"] # [doc = " construct this iterator, but its start position may be different to"] # [doc = " reflect the start of the next search to be executed."] pub fn input < 'i > (& 'i self) -> & 'i Input < 'h > { self . 0 . it . input () } }
    };
}

impl_667!()