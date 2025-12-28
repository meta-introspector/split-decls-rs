macro_rules! deps {
    () => {
        Captures!();
        MatchError!();
        TryCapturesIter!();
        Input!();
    };
}

macro_rules! impl_671 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'h , F > Iterator for TryCapturesIter < 'h , F > where F : FnMut (& Input < '_ > , & mut Captures) -> Result < () , MatchError > , { type Item = Result < Captures , MatchError > ; # [inline] fn next (& mut self) -> Option < Result < Captures , MatchError > > { let TryCapturesIter { ref mut it , ref mut caps , ref mut finder } = * self ; let result = it . try_advance (| input | { (finder) (input , caps) ? ; Ok (caps . get_match ()) }) . transpose () ? ; match result { Ok (_) => Some (Ok (caps . clone ())) , Err (err) => Some (Err (err)) , } } }
    };
}

impl_671!();