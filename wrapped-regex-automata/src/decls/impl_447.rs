macro_rules! deps {
    () => {
        MatchError!();
        Match!();
        TryFindMatches!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        impl < 'r , 'c , 'h > Iterator for TryFindMatches < 'r , 'c , 'h > { type Item = Result < Match , MatchError > ; # [inline] fn next (& mut self) -> Option < Result < Match , MatchError > > { let TryFindMatches { re , ref mut cache , ref mut caps , ref mut it } = * self ; it . try_advance (| input | { re . try_search (cache , input , caps) ? ; Ok (caps . get_match ()) }) . transpose () } }
    };
}

impl_447!();