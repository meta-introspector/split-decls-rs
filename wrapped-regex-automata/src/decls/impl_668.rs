macro_rules! deps {
    () => {
        Match!();
        MatchesIter!();
        Input!();
        MatchError!();
    };
}

macro_rules! impl_668 {
    () => {
        deps!();
        impl < 'h , F > Iterator for MatchesIter < 'h , F > where F : FnMut (& Input < '_ >) -> Result < Option < Match > , MatchError > , { type Item = Match ; # [inline] fn next (& mut self) -> Option < Match > { match self . 0 . next () ? { Ok (m) => Some (m) , Err (err) => panic ! ("unexpected regex find error: {err}\n\
                 to handle find errors, use 'try' or 'search' methods" ,) , } } }
    };
}

impl_668!();