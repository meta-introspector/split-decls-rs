macro_rules! deps {
    () => {
        HalfMatch!();
        Input!();
        MatchError!();
        TryHalfMatchesIter!();
    };
}

macro_rules! impl_657 {
    () => {
        deps!();
        impl < 'h , F > Iterator for TryHalfMatchesIter < 'h , F > where F : FnMut (& Input < '_ >) -> Result < Option < HalfMatch > , MatchError > , { type Item = Result < HalfMatch , MatchError > ; # [inline] fn next (& mut self) -> Option < Result < HalfMatch , MatchError > > { self . it . try_advance_half (& mut self . finder) . transpose () } }
    };
}

impl_657!();