macro_rules! deps {
    () => {
        Input!();
        CapturesIter!();
        MatchError!();
        Captures!();
    };
}

macro_rules! impl_674 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'h , F > Iterator for CapturesIter < 'h , F > where F : FnMut (& Input < '_ > , & mut Captures) -> Result < () , MatchError > , { type Item = Captures ; # [inline] fn next (& mut self) -> Option < Captures > { match self . 0 . next () ? { Ok (m) => Some (m) , Err (err) => panic ! ("unexpected regex captures error: {err}\n\
                 to handle find errors, use 'try' or 'search' methods" ,) , } } }
    };
}

impl_674!()