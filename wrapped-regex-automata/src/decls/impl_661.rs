macro_rules! deps {
    () => {
        HalfMatchesIter!();
        MatchError!();
        HalfMatch!();
        Input!();
    };
}

macro_rules! impl_661 {
    () => {
        deps!();
        impl < 'h , F > Iterator for HalfMatchesIter < 'h , F > where F : FnMut (& Input < '_ >) -> Result < Option < HalfMatch > , MatchError > , { type Item = HalfMatch ; # [inline] fn next (& mut self) -> Option < HalfMatch > { match self . 0 . next () ? { Ok (m) => Some (m) , Err (err) => panic ! ("unexpected regex half find error: {err}\n\
                 to handle find errors, use 'try' or 'search' methods" ,) , } } }
    };
}

impl_661!();