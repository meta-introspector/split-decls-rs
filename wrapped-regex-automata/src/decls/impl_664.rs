macro_rules! deps {
    () => {
        MatchError!();
        Match!();
        TryMatchesIter!();
        Input!();
    };
}

macro_rules! impl_664 {
    () => {
        deps!();
        impl < 'h , F > Iterator for TryMatchesIter < 'h , F > where F : FnMut (& Input < '_ >) -> Result < Option < Match > , MatchError > , { type Item = Result < Match , MatchError > ; # [inline] fn next (& mut self) -> Option < Result < Match , MatchError > > { self . it . try_advance (& mut self . finder) . transpose () } }
    };
}

impl_664!()