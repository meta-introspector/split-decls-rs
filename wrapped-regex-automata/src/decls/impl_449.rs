macro_rules! deps {
    () => {
        TryCapturesMatches!();
        MatchError!();
        Captures!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        impl < 'r , 'c , 'h > Iterator for TryCapturesMatches < 'r , 'c , 'h > { type Item = Result < Captures , MatchError > ; # [inline] fn next (& mut self) -> Option < Result < Captures , MatchError > > { let TryCapturesMatches { re , ref mut cache , ref mut caps , ref mut it } = * self ; let _ = it . try_advance (| input | { re . try_search (cache , input , caps) ? ; Ok (caps . get_match ()) }) . transpose () ? ; if caps . is_match () { Some (Ok (caps . clone ())) } else { None } } }
    };
}

impl_449!()